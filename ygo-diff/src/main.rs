//! Differential testing of the Rust engine against ygopro-core: the same duels (seed, decks,
//! responses) on both, comparing every message and the queried state at every question.

mod core_ffi;
mod question;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ygo_core::api::{NewCardInfo, QueryInfo};
use ygo_core::carddb::CardDb;
use ygo_core::constants::*;
use ygo_core::duel::{Duel, DuelOptions, PlayerOptions};
use ygo_core::processor1::Status;
use ygo_core::script::Scripts;

use core_ffi::{CoreDuel, OcgPlayer};
use question::{random_response, Rng};

const LOCATIONS: [u32; 7] = [LOCATION_MZONE, LOCATION_SZONE, LOCATION_HAND, LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA, LOCATION_DECK];
const QUERY_ALL: u32 = 0x3ffffff;

fn split_messages(buf: &[u8]) -> Vec<Vec<u8>> {
    let mut out = vec![];
    let mut p = 0;
    while p + 4 <= buf.len() {
        let n = u32::from_le_bytes(buf[p..p + 4].try_into().unwrap()) as usize;
        p += 4;
        out.push(buf[p..p + n].to_vec());
        p += n;
    }
    out
}

/// Both buffers around their first difference.
fn diff_hex(a: &[u8], b: &[u8]) -> String {
    let i = a.iter().zip(b.iter()).position(|(x, y)| x != y).unwrap_or(a.len().min(b.len()));
    let lo = i.saturating_sub(24);
    format!("first difference at byte {i} (lengths {} and {})\n  core[{lo}..]: {}\n  rust[{lo}..]: {}", a.len(), b.len(), hex(&a[lo.min(a.len())..]), hex(&b[lo.min(b.len())..]))
}

fn hex(b: &[u8]) -> String {
    let s: Vec<String> = b.iter().take(96).map(|x| format!("{x:02x}")).collect();
    let mut s = s.join(" ");
    if b.len() > 96 {
        s += " ...";
    }
    s
}

thread_local! {
    /// By card code: (activations (MSG_CHAINING), summons (MSG_SUMMONING/SPSUMMONING/FLIPSUMMONING))
    static COVERAGE: std::cell::RefCell<std::collections::HashMap<u32, (u64, u64)>> = std::cell::RefCell::new(std::collections::HashMap::new());
    /// (decisions, total clone time, max clone time, cards, effects)
    static CLONE_STATS: std::cell::RefCell<(u64, Duration, Duration, u64, u64)> = const { std::cell::RefCell::new((0, Duration::ZERO, Duration::ZERO, 0, 0)) };
}

struct Args {
    bench_clone: bool,
    games: u64,
    seed: u64,
    max_decisions: usize,
    verbose: bool,
    /// The share of scripted cards in the decks (in thousandths)
    scripted_permille: usize,
    /// The scripted cards the decks use (default: all translated ones)
    cards: Option<Vec<u32>>,
    /// Extra Deck size (from the scripted Extra Deck monsters)
    extra: usize,
}

fn parse_args() -> Args {
    let mut a = Args { bench_clone: false, games: 10, seed: 1, max_decisions: 3000, verbose: false, scripted_permille: 500, cards: None, extra: 15 };
    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--games" => {
                a.games = argv[i + 1].parse().unwrap();
                i += 1;
            }
            "--seed" => {
                a.seed = argv[i + 1].parse().unwrap();
                i += 1;
            }
            "--max-decisions" => {
                a.max_decisions = argv[i + 1].parse().unwrap();
                i += 1;
            }
            "--scripted" => {
                a.scripted_permille = (argv[i + 1].parse::<f64>().unwrap() * 1000.0) as usize;
                i += 1;
            }
            "--cards" => {
                a.cards = Some(argv[i + 1].split(',').map(|x| x.parse().unwrap()).collect());
                i += 1;
            }
            "--extra" => {
                a.extra = argv[i + 1].parse().unwrap();
                i += 1;
            }
            "-v" | "--verbose" => a.verbose = true,
            "--bench-clone" => a.bench_clone = true,
            x => panic!("unknown argument {x}"),
        }
        i += 1;
    }
    a
}

/// Normal Monsters without a script (their duels need no card functions).
fn vanilla_pool(db: &CardDb) -> Vec<u32> {
    let mut v: Vec<u32> = db
        .cards
        .values()
        .filter(|c| {
            c.type_ & TYPE_MONSTER != 0
                && c.type_ & TYPE_NORMAL != 0
                && c.type_ & (TYPE_TOKEN | TYPE_PENDULUM | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) == 0
                && c.alias == 0
                && c.code < 100_000_000
                && core_ffi::find_script(&format!("c{}.lua", c.code)).is_none()
        })
        .map(|c| c.code)
        .collect();
    v.sort();
    v
}

struct Outcome {
    decisions: usize,
    core_time: Duration,
    rust_time: Duration,
    ended: bool,
    turns: usize,
}

fn run_core(d: &mut CoreDuel) -> (Vec<Vec<u8>>, i32) {
    let mut msgs = vec![];
    loop {
        let st = d.process();
        msgs.extend(split_messages(&d.get_message()));
        if st != 2 {
            return (msgs, st);
        }
    }
}

fn run_rust(d: &mut Duel) -> (Vec<Vec<u8>>, i32) {
    let mut msgs = vec![];
    loop {
        let st = d.ocg_process();
        msgs.extend(split_messages(d.ocg_get_message()));
        if st != Status::Continue {
            return (msgs, st as i32);
        }
    }
}

/// The fields of a `query_location` buffer: (card index, flag, value bytes).
fn decode_location_query(b: &[u8]) -> Vec<(usize, u32, Vec<u8>)> {
    let mut out = vec![];
    let mut p = 4;
    let mut card = 0;
    while p + 2 <= b.len() {
        let len = u16::from_le_bytes([b[p], b[p + 1]]) as usize;
        p += 2;
        if len == 0 {
            card += 1;
            continue;
        }
        let flag = u32::from_le_bytes(b[p..p + 4].try_into().unwrap());
        out.push((card, flag, b[p + 4..p + len].to_vec()));
        p += len;
        if flag == QUERY_END {
            card += 1;
        }
    }
    out
}

fn compare_state(c: &mut CoreDuel, r: &mut Duel) -> Result<(), String> {
    let (fc, fr) = (c.query_field(), r.ocg_query_field());
    if fc != fr {
        return Err(format!("query_field differs: {}", diff_hex(&fc, &fr)));
    }
    for p in 0..2u8 {
        for loc in LOCATIONS {
            let qi = QueryInfo { flags: QUERY_ALL, con: p, loc, seq: 0, overlay_seq: 0 };
            let (a, b) = (c.query_location(&qi), r.ocg_query_location(&qi));
            if a != b {
                let (da, db) = (decode_location_query(&a), decode_location_query(&b));
                if let Some(i) = (0..da.len().max(db.len())).find(|&i| da.get(i) != db.get(i)) {
                    return Err(format!("query_location(player {p}, location {loc:#x}): card {:?} flag {:#x}: core {:?} rust {:?}", da.get(i).map(|x| x.0), da.get(i).or(db.get(i)).map_or(0, |x| x.1), da.get(i).map(|x| hex(&x.2)), db.get(i).map(|x| hex(&x.2))));
                }
                return Err(format!("query_location(player {p}, location {loc:#x}) differs: {}", diff_hex(&a, &b)));
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn play(game: u64, args: &Args, db: &Arc<CardDb>, scripts: &Arc<Scripts>, pool: &[u32], scripted: &[u32], scripted_extra: &[u32], codes: &[u32]) -> Result<Outcome, String> {
    let mut rng = Rng(args.seed.wrapping_mul(1_000_003).wrapping_add(game));
    let seed = [rng.next(), rng.next(), rng.next(), rng.next()];
    let flags = DUEL_MODE_MR5;
    let player = OcgPlayer { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
    let mut c = CoreDuel::new(seed, flags, player);
    let po = PlayerOptions { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
    let mut r = Duel::new(&DuelOptions { seed, flags, team1: po, team2: po }, db.clone(), scripts.clone());
    ygo_cards::load_global_scripts(&mut r);
    for p in 0..2u8 {
        for _ in 0..40 {
            let code = if !scripted.is_empty() && args.scripted_permille > 0 && rng.below(1000) < args.scripted_permille {
                scripted[rng.below(scripted.len())]
            } else {
                pool[rng.below(pool.len())]
            };
            let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_DECK, seq: 0, pos: POS_FACEDOWN_DEFENSE };
            c.new_card(&info);
            r.ocg_new_card(&info);
        }
        if !scripted_extra.is_empty() {
            for _ in 0..args.extra {
                let code = scripted_extra[rng.below(scripted_extra.len())];
                let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_EXTRA, seq: 0, pos: POS_FACEDOWN_DEFENSE };
                c.new_card(&info);
                r.ocg_new_card(&info);
            }
        }
    }
    c.start();
    r.ocg_start_duel();
    let mut out = Outcome { decisions: 0, core_time: Duration::ZERO, rust_time: Duration::ZERO, ended: false, turns: 0 };
    let mut history: Vec<Vec<u8>> = vec![];
    loop {
        let t = Instant::now();
        let (mc, sc) = run_core(&mut c);
        out.core_time += t.elapsed();
        let t = Instant::now();
        let (mr, sr) = run_rust(&mut r);
        out.rust_time += t.elapsed();
        let n = mc.len().max(mr.len());
        for i in 0..n {
            let (a, b) = (mc.get(i), mr.get(i));
            if a != b {
                let mut s = format!("game {game} (seed {:?}), decision {}: message {i} differs\n", seed, out.decisions);
                for h in history.iter().rev().take(6).rev() {
                    s += &format!("  before: {}\n", hex(h));
                }
                for j in i.saturating_sub(3)..i {
                    s += &format!("  same:   {}\n", hex(&mc[j]));
                }
                s += &match (a, b) {
                    (Some(a), Some(b)) => format!("  {}", diff_hex(a, b)),
                    _ => format!("  core:   {}\n  rust:   {}", a.map_or("(none)".into(), |m| hex(m)), b.map_or("(none)".into(), |m| hex(m))),
                };
                return Err(s);
            }
        }
        if sc != sr {
            return Err(format!("game {game}, decision {}: status core {sc} rust {sr}", out.decisions));
        }
        if let Ok(t) = std::env::var("YGO_DIFF_TRACE") {
            let v: Vec<u32> = t.split(',').map(|x| x.parse().unwrap()).collect();
            let qi = QueryInfo { flags: QUERY_CODE | QUERY_STATUS | QUERY_POSITION, con: v[0] as u8, loc: v[1], seq: v[2], overlay_seq: 0 };
            let msgs: Vec<String> = mc.iter().map(|m| m[0].to_string()).collect();
            eprintln!("  trace {}: core {} rust {} [{}]", out.decisions, hex(&c.query(&qi)), hex(&r.ocg_query(&qi)), msgs.join(" "));
        }
        compare_state(&mut c, &mut r).map_err(|e| {
            let types: Vec<String> = mc.iter().map(|m| m[0].to_string()).collect();
            format!("game {game} (seed {:?}), decision {}: {e}\n  messages since the last decision: {}", seed, out.decisions, types.join(" "))
        })?;
        COVERAGE.with(|cov| {
            let mut cov = cov.borrow_mut();
            for m in &mc {
                if m.len() >= 5 {
                    let code = u32::from_le_bytes(m[1..5].try_into().unwrap());
                    match m[0] as u32 {
                        MSG_CHAINING => cov.entry(code).or_default().0 += 1,
                        MSG_SUMMONING | MSG_SPSUMMONING | MSG_FLIPSUMMONING => cov.entry(code).or_default().1 += 1,
                        _ => {}
                    }
                }
            }
        });
        history.extend(mc.iter().cloned());
        if history.len() > 64 {
            history.drain(..history.len() - 64);
        }
        if sc == 0 || mc.iter().any(|m| m[0] as u32 == MSG_WIN) {
            out.ended = true;
            return Ok(out);
        }
        if args.bench_clone {
            let t = Instant::now();
            let n = 200;
            for _ in 0..n {
                std::hint::black_box(r.clone());
            }
            let dt = t.elapsed() / n;
            if out.decisions == 40 {
                let time = |f: &dyn Fn()| {
                    let t = Instant::now();
                    for _ in 0..n {
                        f();
                    }
                    (t.elapsed() / n).as_secs_f64() * 1e6
                };
                eprintln!(
                    "clone parts (us): cards {:.2} effects {:.2} core {:.2} field_effects {:.2} players {:.2} groups {:.2}; sizes: Card {} Effect {} Processor {} Duel {}",
                    time(&|| { std::hint::black_box(r.cards.clone()); }),
                    time(&|| { std::hint::black_box(r.effects.clone()); }),
                    time(&|| { std::hint::black_box(r.core.clone()); }),
                    time(&|| { std::hint::black_box(r.field_effects.clone()); }),
                    time(&|| { std::hint::black_box(r.player.clone()); }),
                    time(&|| { std::hint::black_box(r.groups.clone()); }),
                    std::mem::size_of::<ygo_core::card::Card>(),
                    std::mem::size_of::<ygo_core::effect::Effect>(),
                    std::mem::size_of::<ygo_core::duel::Processor>(),
                    std::mem::size_of::<Duel>(),
                );
            }
            CLONE_STATS.with(|cs| {
                let mut cs = cs.borrow_mut();
                cs.0 += 1;
                cs.1 += dt;
                cs.2 = cs.2.max(dt);
                cs.3 += r.cards.len() as u64;
                cs.4 += r.effects.len() as u64;
            });
        }
        let q = mc.last().ok_or_else(|| format!("game {game}: awaiting without a question"))?;
        let resp = random_response(q, &mut rng, db, codes).ok_or_else(|| format!("game {game}: not a question: {}", hex(q)))?;
        if args.verbose {
            eprintln!("  decision {}: msg {} -> {}", out.decisions, q[0], hex(&resp));
        }
        out.turns += mc.iter().filter(|m| m[0] as u32 == MSG_NEW_TURN).count();
        c.set_response(&resp);
        r.ocg_set_response(&resp);
        out.decisions += 1;
        if out.decisions >= args.max_decisions {
            if args.verbose {
                for p in 0..2u8 {
                    let n: Vec<u32> = [LOCATION_DECK, LOCATION_HAND, LOCATION_MZONE, LOCATION_SZONE, LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA].iter().map(|&l| r.ocg_query_count(p, l)).collect();
                    eprintln!("  player {p}: lp {} deck/hand/mzone/szone/grave/removed/extra {:?}", r.player[p as usize].lp, n);
                }
            }
            return Ok(out);
        }
    }
}

fn main() {
    let args = parse_args();
    let root = std::env::var("YGO_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let root = root.canonicalize().unwrap();
    let db = Arc::new(CardDb::load(&root.join("external").join("BabelCDB")).expect("card database"));
    core_ffi::init(&root.join("build").join("libocgcore.so"), &root, db.clone());
    let scripts = Arc::new(ygo_cards::scripts());
    let pool = vanilla_pool(&db);
    let scripted: Vec<u32> = args.cards.clone().unwrap_or_else(|| ygo_cards::cards::ALL.iter().map(|x| x.0).collect());
    for c in &scripted {
        assert!(scripts.get(*c).is_some() || core_ffi::find_script(&format!("c{c}.lua")).is_none(), "card {c} has no translated script");
    }
    let is_extra = |c: &u32| db.cards.get(c).is_some_and(|x| x.type_ & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) != 0);
    let scripted_extra: Vec<u32> = scripted.iter().copied().filter(is_extra).collect();
    let scripted: Vec<u32> = scripted.iter().copied().filter(|c| !is_extra(c)).collect();
    let mut codes: Vec<u32> = db.cards.keys().copied().collect();
    codes.sort();
    eprintln!("{} vanilla monsters, {} scripted cards, {} scripted Extra Deck monsters", pool.len(), scripted.len(), scripted_extra.len());
    let (mut ok, mut decisions, mut ended) = (0, 0, 0);
    let (mut tc, mut tr) = (Duration::ZERO, Duration::ZERO);
    for g in 0..args.games {
        match play(g, &args, &db, &scripts, &pool, &scripted, &scripted_extra, &codes) {
            Ok(o) => {
                ok += 1;
                decisions += o.decisions;
                ended += o.ended as u64;
                if (g + 1) % 10 == 0 {
                    eprintln!("{} games identical", g + 1);
                }
                tc += o.core_time;
                tr += o.rust_time;
                if args.verbose {
                    eprintln!("game {g}: {} decisions, {} turns, ended {}", o.decisions, o.turns, o.ended);
                }
            }
            Err(e) => {
                println!("MISMATCH {e}");
                break;
            }
        }
    }
    CLONE_STATS.with(|cs| {
        let cs = cs.borrow();
        if cs.0 > 0 {
            println!("clone: {} states, mean {:.1} us, max {:.1} us (mean {} cards, {} effects)", cs.0, cs.1.as_secs_f64() * 1e6 / cs.0 as f64, cs.2.as_secs_f64() * 1e6, cs.3 / cs.0, cs.4 / cs.0);
        }
    });
    COVERAGE.with(|cov| {
        let cov = cov.borrow();
        let mut v: Vec<String> = scripted.iter().chain(scripted_extra.iter()).map(|c| {
            let (a, s) = cov.get(c).copied().unwrap_or_default();
            format!("{c}: {a} activations, {s} summons")
        }).collect();
        v.sort();
        if !v.is_empty() && v.len() <= 50 {
            println!("coverage of the scripted cards:\n  {}", v.join("\n  "));
        }
    });
    println!("{ok}/{} games identical ({ended} ended, the others reached --max-decisions), {decisions} decisions; core {:.3}s, rust {:.3}s", args.games, tc.as_secs_f64(), tr.as_secs_f64());
}
