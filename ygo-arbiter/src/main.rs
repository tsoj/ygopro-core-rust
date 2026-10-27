//! fastchess-style match between two Yu-Gi-Oh bots.
//!
//!   ygo-arbiter --bot new="my-bot --hash 64" --bot base="other-bot" \
//!       --book book.jsonl --sprt 0 5 --tc 10+0.1 -n 10000 -c 32
//!
//! Openings come from a book (a JSON line per opening: its two decks); every opening is played
//! twice, the bots swapping seats. Bots are commands speaking the bot protocol
//! (PROTOCOL.md).
//!
//! `--nps-nodes N` (NPS-adjusted node limit): the second bot (the base) searches N nodes per
//! decision, the first N times the ratio of their nodes per second, measured over the match's
//! searches (`matchrun::NpsNodes`): a fixed-node test that charges a slower bot for its speed.
//!
//! `--resume RECORDS` continues a stopped match: the finished pairs of RECORDS (an earlier run's
//! `--records`, the same bot names) count towards the statistics and the SPRT, their openings
//! are not played again, and the records file of this run starts with them.

use std::collections::HashMap;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Instant;

use ygo_arbiter::game::BotError;
use ygo_arbiter::matchrun::{BotSpec, MatchStats, NpsNodes, Opening, load_book_lines, play_opening, sprt_bounds};

struct Args {
    bots: Vec<BotSpec>,
    openings: usize,
    tc: Option<(f64, f64)>,
    max_turns: u32,
    sprt: Option<(f64, f64)>,
    alpha: f64,
    beta: f64,
    concurrency: usize,
    records: Option<PathBuf>,
    book: Option<PathBuf>,
    book_min: i64,
    shuffle: Option<u64>,
    pool: usize,
    nps_nodes: Option<u64>,
    resume: Option<PathBuf>,
}

fn usage(msg: &str) -> ! {
    eprintln!("ygo-arbiter: {msg}");
    eprintln!(
        "usage: ygo-arbiter --bot NAME=COMMAND --bot NAME=COMMAND --book BOOK [-n N] [--tc BASE+INC] \
         [--sprt ELO0 ELO1] [--alpha A] [--beta B] [-c J] [--records PATH] [--book-min N] [--shuffle SEED] [--pool N] [--max-turns N] [--nps-nodes N] [--resume RECORDS] [--option BOT.NAME=VALUE ...]"
    );
    std::process::exit(2)
}

/// Fisher-Yates with splitmix64: each test draws its own sample of the book (reproducible by seed).
fn shuffle<T>(v: &mut [T], seed: u64) {
    let mut x = seed;
    let mut next = || {
        x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    for i in (1..v.len()).rev() {
        v.swap(i, (next() % (i as u64 + 1)) as usize);
    }
}

/// The finished pairs of an earlier run's records, in their order: the opening's seed, the record
/// lines and `name`'s scores.
fn finished_pairs(path: &PathBuf, name: &str) -> Vec<(i64, Vec<String>, Vec<f64>)> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| usage(&format!("resume: {e}")));
    let mut pairs: Vec<(i64, Vec<String>, Vec<f64>)> = Vec::new();
    let mut at: HashMap<i64, usize> = HashMap::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let r: serde_json::Value = serde_json::from_str(line).unwrap_or_else(|e| usage(&format!("resume: {e}")));
        let seed = r["seed"].as_i64().unwrap_or_else(|| usage("resume: a record without a seed"));
        let seat = r["agents"].as_array().and_then(|a| a.iter().position(|n| n.as_str() == Some(name)));
        let seat = seat.unwrap_or_else(|| usage(&format!("resume: a record without the bot {name}")));
        let score = match r["result"]["winner"].as_u64() {
            None => 0.5,
            Some(w) if w as usize == seat => 1.0,
            Some(_) => 0.0,
        };
        let k = *at.entry(seed).or_insert_with(|| {
            pairs.push((seed, Vec::new(), Vec::new()));
            pairs.len() - 1
        });
        pairs[k].1.push(line.to_string());
        pairs[k].2.push(score);
    }
    pairs.retain(|p| p.2.len() == 2);
    pairs
}

fn parse_args() -> Args {
    let mut a = Args {
        bots: Vec::new(),
        openings: 100,
        tc: None,
        max_turns: 100,
        sprt: None,
        alpha: 0.05,
        beta: 0.05,
        concurrency: std::thread::available_parallelism().map_or(1, |n| n.get()),
        records: None,
        book: None,
        book_min: 2,
        shuffle: None,
        pool: usize::MAX,
        nps_nodes: None,
        resume: None,
    };
    let mut options = Vec::new();
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let next = |i: &mut usize, what: &str| -> String {
        *i += 1;
        argv.get(*i).cloned().unwrap_or_else(|| usage(&format!("{what} needs a value")))
    };
    let num = |s: String, what: &str| -> f64 { s.parse().unwrap_or_else(|_| usage(&format!("bad {what}: {s}"))) };
    while i < argv.len() {
        let arg = argv[i].clone();
        match arg.as_str() {
            "--bot" => {
                let v = next(&mut i, "--bot");
                a.bots.push(BotSpec::parse(&v).unwrap_or_else(|| usage("expected NAME=COMMAND")));
            }
            "--option" => {
                let v = next(&mut i, "--option");
                let Some((bot, (key, value))) = v.split_once('.').and_then(|(e, rest)| Some((e, rest.split_once('=')?))) else {
                    usage("expected --option BOT.NAME=VALUE")
                };
                options.push((bot.to_string(), key.to_string(), value.to_string()));
            }
            "-n" | "--openings" => a.openings = num(next(&mut i, &arg), "-n") as usize,
            "--tc" => {
                let v = next(&mut i, "--tc");
                let (base, inc) = v.split_once('+').unwrap_or((&v, "0"));
                a.tc = Some((num(base.into(), "--tc"), num(inc.into(), "--tc")));
            }
            "--max-turns" => a.max_turns = num(next(&mut i, &arg), &arg) as u32,
            "--sprt" => {
                let e0 = num(next(&mut i, &arg), &arg);
                a.sprt = Some((e0, num(next(&mut i, &arg), &arg)));
            }
            "--alpha" => a.alpha = num(next(&mut i, &arg), &arg),
            "--beta" => a.beta = num(next(&mut i, &arg), &arg),
            "-c" | "--concurrency" => a.concurrency = num(next(&mut i, &arg), &arg) as usize,
            "--records" => a.records = Some(next(&mut i, &arg).into()),
            "--book" => a.book = Some(next(&mut i, &arg).into()),
            "--book-min" => a.book_min = num(next(&mut i, &arg), &arg) as i64,
            // (only the book's first N lines: the eval's training pairs come from later ones)
            "--pool" => a.pool = num(next(&mut i, &arg), &arg) as usize,
            "--resume" => a.resume = Some(next(&mut i, &arg).into()),
            "--nps-nodes" => a.nps_nodes = Some(num(next(&mut i, &arg), &arg) as u64),
            "--shuffle" => a.shuffle = Some(next(&mut i, &arg).parse().unwrap_or_else(|_| usage("--shuffle takes an integer seed"))),
            _ => usage(&format!("unknown argument {arg}")),
        }
        i += 1;
    }
    if a.bots.len() != 2 {
        usage("exactly two bots are required");
    }
    for (bot, key, value) in options {
        let Some(spec) = a.bots.iter_mut().find(|e| e.name == bot) else { usage(&format!("--option for {bot}, which is no bot")) };
        spec.options.push((key, value));
    }
    if a.book.is_none() {
        usage("--book is required");
    }
    a
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let args = parse_args();
    if let Err(e) = ygo_arbiter::game::init(None) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    // (each bot once before the match: its identity, and its options checked)
    for spec in &args.bots {
        match spec.start() {
            Ok(e) => println!("bot {}: {}", spec.name, serde_json::to_string(&e.hello.id).unwrap()),
            Err(BotError::Failed(e) | BotError::Forfeit(e)) => {
                eprintln!("ygo-arbiter: {e}");
                std::process::exit(1);
            }
        }
    }
    let mut openings: Vec<Opening> = load_book_lines(args.book.as_ref().unwrap(), args.book_min, args.pool).unwrap_or_else(|e| usage(&format!("book: {e}")));
    if let Some(seed) = args.shuffle {
        shuffle(&mut openings, seed);
        println!("book shuffled with seed {seed}");
    }
    openings.truncate(args.openings);
    let (a, b) = (args.bots[0].clone(), args.bots[1].clone());
    println!("{} vs {}: {} openings x 2 games", a.name, b.name, openings.len());
    let nps = args.nps_nodes.map(|n| Arc::new(NpsNodes::new(n)));
    if let Some(n) = args.nps_nodes {
        println!("NPS-adjusted node limit: {} searches {n} nodes per decision, {} {n} times their speed ratio", b.name, a.name);
    }
    let nps_line = |nps: &Option<Arc<NpsNodes>>| {
        nps.as_ref().map_or(String::new(), |p| {
            let [x, y] = p.nps();
            format!("  nps {x:.0} / {y:.0}, nodes {} / {}", p.limit(0), p.limit(1))
        })
    };
    let mut out = args.records.as_ref().map(|p| std::fs::File::create(p).unwrap_or_else(|e| usage(&format!("records: {e}"))));
    let (lo, hi) = args.sprt.map(|_| sprt_bounds(args.alpha, args.beta)).unwrap_or((0.0, 0.0));
    let mut stats = MatchStats::default();
    if let Some(path) = &args.resume {
        let pairs = finished_pairs(path, &a.name);
        let done: std::collections::HashSet<i64> = pairs.iter().map(|p| p.0).collect();
        for (_, lines, scores) in &pairs {
            stats.add_pair(scores);
            if let Some(f) = out.as_mut() {
                for l in lines {
                    let _ = writeln!(f, "{l}");
                }
            }
        }
        openings.retain(|o| !done.contains(&o.seed));
        println!("resumed {} pairs from {}: {}", pairs.len(), path.display(), stats.summary(args.sprt));
        println!("{} openings left", openings.len());
        if let Some((e0, e1)) = args.sprt {
            let llr = stats.llr(e0, e1);
            if llr <= lo || llr >= hi {
                println!("SPRT finished: {} (LLR {llr:.2}, bounds [{lo:.2}, {hi:.2}])", if llr >= hi { "H1 accepted" } else { "H0 accepted" });
                println!("final: {} errors: 0", stats.summary(args.sprt));
                std::process::exit(0);
            }
        }
    }
    let openings = Arc::new(openings);
    let next_task = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();
    let n = openings.len();
    for _ in 0..args.concurrency.min(n).max(1) {
        let (openings, next_task, stop, tx, a, b, nps) = (openings.clone(), next_task.clone(), stop.clone(), tx.clone(), a.clone(), b.clone(), nps.clone());
        let (tc, max_turns) = (args.tc, args.max_turns);
        std::thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                let i = next_task.fetch_add(1, Ordering::Relaxed);
                let Some(opening) = openings.get(i) else { break };
                let res = catch_unwind(AssertUnwindSafe(|| play_opening(opening, &a, &b, tc, max_turns, nps.as_deref())))
                    .unwrap_or_else(|p| Err(format!("crash: {}", p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default())));
                if tx.send((opening.seed, res)).is_err() {
                    break;
                }
            }
        });
    }
    drop(tx);
    let mut errors = 0;
    let t0 = Instant::now();
    for (i, (seed, res)) in rx.iter().enumerate() {
        let i = i + 1;
        let games = match res {
            Ok(g) => g,
            Err(e) => {
                errors += 1;
                println!("error: opening {seed}: {e}");
                continue;
            }
        };
        stats.add_pair(&games.iter().map(|g| g.score).collect::<Vec<_>>());
        if let Some(f) = out.as_mut() {
            for g in &games {
                let _ = writeln!(f, "{}", serde_json::to_string(&g.record).unwrap());
            }
            let _ = f.flush();
        }
        if i % 10 == 0 || i == n {
            println!("[{:6.0}s] {}{}", t0.elapsed().as_secs_f64(), stats.summary(args.sprt), nps_line(&nps));
        }
        if let Some((e0, e1)) = args.sprt {
            let llr = stats.llr(e0, e1);
            if llr <= lo || llr >= hi {
                println!(
                    "SPRT finished: {} (LLR {llr:.2}, bounds [{lo:.2}, {hi:.2}])",
                    if llr >= hi { "H1 accepted" } else { "H0 accepted" }
                );
                stop.store(true, Ordering::Relaxed);
                break;
            }
        }
        let _ = std::io::stdout().flush();
    }
    println!("final: {} errors: {errors}{}", stats.summary(args.sprt), nps_line(&nps));
    let _ = std::io::stdout().flush();
    // (games still running are abandoned, as the Python arbiter does; their bots end with
    // this process)
    std::process::exit(0);
}
