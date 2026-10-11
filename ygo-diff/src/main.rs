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
    /// Two .ydk decks to play against each other (seats alternate between games)
    decks: Option<Vec<Deck>>,
    /// Directory of recorded core games (`--cache DIR`): see `cache`
    cache: Option<PathBuf>,
}

/// A deck from a .ydk file.
#[derive(Clone, Default)]
struct Deck {
    main: Vec<u32>,
    extra: Vec<u32>,
}
fn load_ydk(path: &str) -> Deck {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut d = Deck::default();
    let mut section = "";
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.starts_with('!') {
            section = line;
            continue;
        }
        if let Ok(code) = line.parse::<u32>() {
            match section {
                "#main" => d.main.push(code),
                "#extra" => d.extra.push(code),
                _ => {}
            }
        }
    }
    d
}

fn parse_args() -> Args {
    let mut a = Args { bench_clone: false, games: 10, seed: 1, max_decisions: 3000, verbose: false, scripted_permille: 500, cards: None, extra: 15, decks: None, cache: None };
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
            "--decks" => {
                a.decks = Some(argv[i + 1].split(',').map(load_ydk).collect());
                assert_eq!(a.decks.as_ref().unwrap().len(), 2, "--decks takes two .ydk files");
                i += 1;
            }
            "--extra" => {
                a.extra = argv[i + 1].parse().unwrap();
                i += 1;
            }
            "--cache" => {
                a.cache = Some(PathBuf::from(&argv[i + 1]));
                i += 1;
            }
            "-h" | "--help" => {
                println!("ygo-diff [--games N] [--seed S] [--max-decisions N] [--cards a,b,..] [--scripted SHARE] [--extra N] [--decks a.ydk,b.ydk] [--cache DIR] [-v] [--bench-clone]");
                std::process::exit(0);
            }
            "-v" | "--verbose" => a.verbose = true,
            "--bench-clone" => a.bench_clone = true,
            x => panic!("unknown argument {x}"),
        }
        i += 1;
    }
    a
}

/// Recorded core games. A game is fully determined by the core, the scripts, the decks and the
/// seed, so the core's side of it can be stored: per decision, a hash of the core's messages, its
/// status and a hash of the queried state. A cached game then runs only the Rust engine; any
/// difference reruns the game with the core (for the readable diff, or to replace a stale record).
mod cache {
    use std::path::{Path, PathBuf};

    /// Change when the recorded content or the random responses change.
    const VERSION: u64 = 2;

    #[derive(Default, Clone)]
    pub struct Record {
        /// (messages hash, status, state hash) per decision
        pub steps: Vec<(u64, i32, u64)>,
        /// The game ended (else it stopped at --max-decisions)
        pub complete: bool,
        /// The core ran out of its script instruction budget at the decision after the last step
        /// (stored as `complete` byte 2): the game is skipped from there on
        pub budget: bool,
    }

    /// A fast 64-bit hash step (8 bytes at a time; not FNV for the full words, the name stays).
    pub fn fnv(h: &mut u64, b: &[u8]) {
        let mut chunks = b.chunks_exact(8);
        for c in &mut chunks {
            *h = (*h ^ u64::from_le_bytes(c.try_into().unwrap())).wrapping_mul(0x9e3779b97f4a7c15).rotate_left(29);
        }
        for &x in chunks.remainder() {
            *h = (*h ^ x as u64).wrapping_mul(0x100000001b3);
        }
    }
    pub const FNV0: u64 = 0xcbf29ce484222325;

    pub fn hash_msgs(ms: &[Vec<u8>]) -> u64 {
        let mut h = FNV0;
        for m in ms {
            fnv(&mut h, &(m.len() as u32).to_le_bytes());
            fnv(&mut h, m);
        }
        h
    }

    pub struct Cache {
        dir: PathBuf,
    }
    impl Cache {
        /// `key`: everything besides the core that determines the games (decks, seed, ...)
        pub fn new(base: &Path, core_lib: &Path, key: &[u8]) -> Cache {
            let mut h = FNV0;
            fnv(&mut h, &VERSION.to_le_bytes());
            let m = std::fs::metadata(core_lib).expect("core library");
            fnv(&mut h, &m.len().to_le_bytes());
            let t = m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs());
            fnv(&mut h, &t.to_le_bytes());
            fnv(&mut h, key);
            let dir = base.join(format!("{h:016x}"));
            std::fs::create_dir_all(&dir).expect("cache directory");
            Cache { dir }
        }
        pub fn load(&self, game: u64) -> Option<Record> {
            let b = std::fs::read(self.dir.join(format!("{game}.bin"))).ok()?;
            let n = u32::from_le_bytes(b.get(1..5)?.try_into().ok()?) as usize;
            if b.len() != 5 + n * 20 {
                return None;
            }
            let steps = (0..n)
                .map(|i| {
                    let p = 5 + i * 20;
                    (u64::from_le_bytes(b[p..p + 8].try_into().unwrap()), i32::from_le_bytes(b[p + 8..p + 12].try_into().unwrap()), u64::from_le_bytes(b[p + 12..p + 20].try_into().unwrap()))
                })
                .collect();
            Some(Record { steps, complete: b[0] != 0, budget: b[0] == 2 })
        }
        pub fn store(&self, game: u64, r: &Record) {
            let mut b = vec![if r.budget { 2 } else { r.complete as u8 }];
            b.extend((r.steps.len() as u32).to_le_bytes());
            for &(m, s, q) in &r.steps {
                b.extend(m.to_le_bytes());
                b.extend(s.to_le_bytes());
                b.extend(q.to_le_bytes());
            }
            let tmp = self.dir.join(format!("{game}.tmp"));
            if std::fs::write(&tmp, &b).is_ok() {
                let _ = std::fs::rename(&tmp, self.dir.join(format!("{game}.bin")));
            }
        }
    }
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
    /// The core's side of the game (empty when it came from the cache)
    record: cache::Record,
    /// By card code: (activations, summons)
    cov: std::collections::HashMap<u32, (u64, u64)>,
    decisions: usize,
    core_time: Duration,
    rust_time: Duration,
    ended: bool,
    turns: usize,
    /// Skipped at the core's script budget (see `core_ffi::SCRIPT_BUDGET_EXCEEDED`)
    budget: bool,
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

/// The hash of the state `compare_state` compares (the Rust side).
fn rust_state_hash(r: &mut Duel) -> u64 {
    let mut h = cache::FNV0;
    cache::fnv(&mut h, &r.ocg_query_field());
    for p in 0..2u8 {
        for loc in LOCATIONS {
            let qi = QueryInfo { flags: QUERY_ALL, con: p, loc, seq: 0, overlay_seq: 0 };
            let b = r.ocg_query_location(&qi);
            cache::fnv(&mut h, &(b.len() as u32).to_le_bytes());
            cache::fnv(&mut h, &b);
        }
    }
    h
}

/// Compares the queried state; returns its hash (as `rust_state_hash`).
fn compare_state(c: &mut CoreDuel, r: &mut Duel) -> Result<u64, String> {
    let (fc, fr) = (c.query_field(), r.ocg_query_field());
    if fc != fr {
        return Err(format!("query_field differs: {}", diff_hex(&fc, &fr)));
    }
    let mut h = cache::FNV0;
    cache::fnv(&mut h, &fc);
    for p in 0..2u8 {
        for loc in LOCATIONS {
            let qi = QueryInfo { flags: QUERY_ALL, con: p, loc, seq: 0, overlay_seq: 0 };
            let (a, b) = (c.query_location(&qi), r.ocg_query_location(&qi));
            cache::fnv(&mut h, &(a.len() as u32).to_le_bytes());
            cache::fnv(&mut h, &a);
            if a != b {
                let (da, db) = (decode_location_query(&a), decode_location_query(&b));
                if let Some(i) = (0..da.len().max(db.len())).find(|&i| da.get(i) != db.get(i)) {
                    return Err(format!("query_location(player {p}, location {loc:#x}): card {:?} flag {:#x}: core {:?} rust {:?}", da.get(i).map(|x| x.0), da.get(i).or(db.get(i)).map_or(0, |x| x.1), da.get(i).map(|x| hex(&x.2)), db.get(i).map(|x| hex(&x.2))));
                }
                return Err(format!("query_location(player {p}, location {loc:#x}) differs: {}", diff_hex(&a, &b)));
            }
        }
    }
    Ok(h)
}

#[allow(clippy::too_many_arguments)]
/// One game. With `cached`, only the Rust engine runs, checked against the record; `Err(None)`
/// means it disagreed (or the record ended early): rerun with the core.
fn play(game: u64, args: &Args, db: &Arc<CardDb>, scripts: &Arc<Scripts>, pool: &[u32], scripted: &[u32], scripted_extra: &[u32], codes: &[u32], cached: Option<&cache::Record>) -> Result<Outcome, Option<String>> {
    let mut rng = Rng(args.seed.wrapping_mul(1_000_003).wrapping_add(game));
    let seed = [rng.next(), rng.next(), rng.next(), rng.next()];
    let flags = DUEL_MODE_MR5;
    let player = OcgPlayer { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
    let mut core = if cached.is_none() { Some(CoreDuel::new(seed, flags, player)) } else { None };
    let po = PlayerOptions { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
    let mut r = Duel::new(&DuelOptions { seed, flags, team1: po, team2: po }, db.clone(), scripts.clone());
    ygo_cards::load_global_scripts(&mut r);
    if let Some(decks) = &args.decks {
        // (the decks change seats every game)
        for p in 0..2u8 {
            let deck = &decks[((p as u64 + game) % 2) as usize];
            for &code in &deck.main {
                let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_DECK, seq: 0, pos: POS_FACEDOWN_DEFENSE };
                if let Some(c) = core.as_mut() {
                    c.new_card(&info);
                }
                r.ocg_new_card(&info);
            }
            for &code in &deck.extra {
                let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_EXTRA, seq: 0, pos: POS_FACEDOWN_DEFENSE };
                if let Some(c) = core.as_mut() {
                    c.new_card(&info);
                }
                r.ocg_new_card(&info);
            }
        }
    }
    for p in 0..2u8 {
        if args.decks.is_some() {
            break;
        }
        for _ in 0..40 {
            let code = if !scripted.is_empty() && args.scripted_permille > 0 && rng.below(1000) < args.scripted_permille {
                scripted[rng.below(scripted.len())]
            } else {
                pool[rng.below(pool.len())]
            };
            let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_DECK, seq: 0, pos: POS_FACEDOWN_DEFENSE };
            if let Some(c) = core.as_mut() {
                c.new_card(&info);
            }
            r.ocg_new_card(&info);
        }
        if !scripted_extra.is_empty() {
            for _ in 0..args.extra {
                let code = scripted_extra[rng.below(scripted_extra.len())];
                let info = NewCardInfo { team: p, duelist: 0, code, con: p, loc: LOCATION_EXTRA, seq: 0, pos: POS_FACEDOWN_DEFENSE };
                if let Some(c) = core.as_mut() {
                    c.new_card(&info);
                }
                r.ocg_new_card(&info);
            }
        }
    }
    if let Some(c) = core.as_mut() {
        c.start();
    }
    r.ocg_start_duel();
    let mut out = Outcome { record: cache::Record::default(), cov: Default::default(), decisions: 0, core_time: Duration::ZERO, rust_time: Duration::ZERO, ended: false, turns: 0, budget: false };
    core_ffi::SCRIPT_BUDGET_EXCEEDED.store(false, std::sync::atomic::Ordering::Relaxed);
    let mut history: Vec<Vec<u8>> = vec![];
    loop {
        if let Some(rec) = cached {
            if rec.budget && out.decisions == rec.steps.len() {
                out.budget = true;
                return Ok(out);
            }
        }
        // the core first: when it runs out of its script budget, the Rust engine's step (often the
        // same exponential search, without a budget) is not run
        let core_step = core.as_mut().map(|c| {
            let t = Instant::now();
            let x = run_core(c);
            out.core_time += t.elapsed();
            x
        });
        if core_ffi::SCRIPT_BUDGET_EXCEEDED.load(std::sync::atomic::Ordering::Relaxed) {
            out.budget = true;
            out.record.budget = true;
            out.record.complete = true;
            return Ok(out);
        }
        let t = Instant::now();
        let (mr, sr) = run_rust(&mut r);
        out.rust_time += t.elapsed();
        let Some(c) = core.as_mut() else {
            // cached: check the messages and the state against the record
            let step = cached.unwrap().steps.get(out.decisions).ok_or(None)?;
            if cache::hash_msgs(&mr) != step.0 || sr != step.1 || rust_state_hash(&mut r) != step.2 {
                return Err(None);
            }
            let (more, resp) = after_step(game, args, db, codes, &mut r, &mut rng, &mut out, &mr, sr)?;
            if !more {
                return Ok(out);
            }
            r.ocg_set_response(&resp.unwrap());
            continue;
        };
        let (mc, sc) = core_step.unwrap();
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
                return Err(Some(s));
            }
        }
        if sc != sr {
            return Err(Some(format!("game {game}, decision {}: status core {sc} rust {sr}", out.decisions)));
        }
        if let Ok(t) = std::env::var("YGO_DIFF_TRACE") {
            let v: Vec<u32> = t.split(',').map(|x| x.parse().unwrap()).collect();
            let qi = QueryInfo { flags: QUERY_CODE | QUERY_STATUS | QUERY_POSITION, con: v[0] as u8, loc: v[1], seq: v[2], overlay_seq: 0 };
            let msgs: Vec<String> = mc.iter().map(|m| m[0].to_string()).collect();
            eprintln!("  trace {}: core {} rust {} [{}]", out.decisions, hex(&c.query(&qi)), hex(&r.ocg_query(&qi)), msgs.join(" "));
        }
        let state = compare_state(c, &mut r).map_err(|e| {
            let types: Vec<String> = mc.iter().map(|m| m[0].to_string()).collect();
            Some(format!("game {game} (seed {:?}), decision {}: {e}\n  messages since the last decision: {}", seed, out.decisions, types.join(" ")))
        })?;
        out.record.steps.push((cache::hash_msgs(&mc), sc, state));
        history.extend(mc.iter().cloned());
        if history.len() > 64 {
            history.drain(..history.len() - 64);
        }
        let (more, resp) = after_step(game, args, db, codes, &mut r, &mut rng, &mut out, &mr, sr)?;
        if !more {
            out.record.complete = out.ended;
            return Ok(out);
        }
        let resp = resp.unwrap();
        c.set_response(&resp);
        r.ocg_set_response(&resp);
    }
}

/// After a decision's messages matched: coverage, the end check and the random response.
/// Returns whether the game goes on, and the response to set on both engines.
#[allow(clippy::too_many_arguments)]
fn after_step(game: u64, args: &Args, db: &Arc<CardDb>, codes: &[u32], r: &mut Duel, rng: &mut Rng, out: &mut Outcome, mc: &[Vec<u8>], sc: i32) -> Result<(bool, Option<Vec<u8>>), Option<String>> {
    {
        {
            for m in mc {
                if m.len() >= 5 {
                    let code = u32::from_le_bytes(m[1..5].try_into().unwrap());
                    match m[0] as u32 {
                        MSG_CHAINING => out.cov.entry(code).or_default().0 += 1,
                        MSG_SUMMONING | MSG_SPSUMMONING | MSG_FLIPSUMMONING => out.cov.entry(code).or_default().1 += 1,
                        _ => {}
                    }
                }
            }
        }
        if sc == 0 || mc.iter().any(|m| m[0] as u32 == MSG_WIN) {
            out.ended = true;
            return Ok((false, None));
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
        let q = mc.last().ok_or_else(|| Some(format!("game {game}: awaiting without a question")))?;
        let resp = random_response(q, rng, db, codes).ok_or_else(|| Some(format!("game {game}: not a question: {}", hex(q))))?;
        if args.verbose {
            eprintln!("  decision {}: msg {} -> {}", out.decisions, q[0], hex(&resp));
        }
        out.turns += mc.iter().filter(|m| m[0] as u32 == MSG_NEW_TURN).count();
        out.decisions += 1;
        if out.decisions >= args.max_decisions {
            if args.verbose {
                for p in 0..2u8 {
                    let n: Vec<u32> = [LOCATION_DECK, LOCATION_HAND, LOCATION_MZONE, LOCATION_SZONE, LOCATION_GRAVE, LOCATION_REMOVED, LOCATION_EXTRA].iter().map(|&l| r.ocg_query_count(p, l)).collect();
                    eprintln!("  player {p}: lp {} deck/hand/mzone/szone/grave/removed/extra {:?}", r.player[p as usize].lp, n);
                }
            }
            return Ok((false, None));
        }
        Ok((true, Some(resp)))
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
    if let Some(decks) = &args.decks {
        let mut missing = Vec::new();
        for c in decks.iter().flat_map(|d| d.main.iter().chain(d.extra.iter())) {
            let script = db.cards.get(c).map_or(*c, |x| if x.alias != 0 && x.alias < c + 10 && *c < x.alias + 10 { x.alias } else { *c });
            if scripts.get(script).is_none() && core_ffi::find_script(&format!("c{script}.lua")).is_some() && !missing.contains(&script) {
                missing.push(script);
            }
        }
        assert!(missing.is_empty(), "cards of the decks without a translated script: {missing:?}");
    }
    let is_extra = |c: &u32| db.cards.get(c).is_some_and(|x| x.type_ & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK) != 0);
    let scripted_extra: Vec<u32> = scripted.iter().copied().filter(is_extra).collect();
    let scripted: Vec<u32> = scripted.iter().copied().filter(|c| !is_extra(c)).collect();
    let mut codes: Vec<u32> = db.cards.keys().copied().collect();
    codes.sort();
    eprintln!("{} vanilla monsters, {} scripted cards, {} scripted Extra Deck monsters", pool.len(), scripted.len(), scripted_extra.len());
    let (mut ok, mut decisions, mut ended, mut budget) = (0, 0, 0, 0);
    let (mut tc, mut tr) = (Duration::ZERO, Duration::ZERO);
    let cache = args.cache.as_ref().map(|dir| {
        let mut key = format!("seed {} ", args.seed);
        match &args.decks {
            Some(decks) => {
                for d in decks {
                    key += &format!("{:?} {:?} ", d.main, d.extra);
                }
            }
            None => key += &format!("{:?} {:?} {} {} {}", scripted, scripted_extra, args.scripted_permille, args.extra, pool.len()),
        }
        cache::Cache::new(dir, &root.join("build").join("libocgcore.so"), key.as_bytes())
    });
    let (mut cached_games, mut stale) = (0, 0);
    for g in 0..args.games {
        let rec = cache.as_ref().and_then(|c| c.load(g)).filter(|r| r.complete || r.steps.len() >= args.max_decisions);
        let res = match rec.as_ref().map(|rec| play(g, &args, &db, &scripts, &pool, &scripted, &scripted_extra, &codes, Some(rec))) {
            Some(Ok(o)) => {
                cached_games += 1;
                Ok(o)
            }
            other => {
                let r = play(g, &args, &db, &scripts, &pool, &scripted, &scripted_extra, &codes, None);
                if let (Some(c), Ok(o)) = (&cache, &r) {
                    c.store(g, &o.record);
                    stale += other.is_some() as u64;
                }
                r
            }
        };
        match res.map_err(|e| e.unwrap()) {
            Ok(o) => {
                COVERAGE.with(|cov| {
                    let mut cov = cov.borrow_mut();
                    for (k, v) in &o.cov {
                        let e = cov.entry(*k).or_default();
                        e.0 += v.0;
                        e.1 += v.1;
                    }
                });
                if o.budget {
                    budget += 1;
                    println!("game {g}: skipped at decision {}: the core ran out of its script instruction budget", o.decisions);
                } else {
                    ok += 1;
                }
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
        let deck_cards: Vec<u32> = match &args.decks {
            Some(decks) => {
                let mut v: Vec<u32> = decks.iter().flat_map(|d| d.main.iter().chain(d.extra.iter()).copied()).collect();
                v.sort();
                v.dedup();
                v
            }
            None => scripted.iter().chain(scripted_extra.iter()).copied().collect(),
        };
        let mut v: Vec<String> = deck_cards.iter().map(|c| {
            let (a, s) = cov.get(c).copied().unwrap_or_default();
            format!("{c}: {a} activations, {s} summons")
        }).collect();
        v.sort();
        if !v.is_empty() && (v.len() <= 50 || args.decks.is_some()) {
            println!("coverage of the scripted cards:\n  {}", v.join("\n  "));
        }
    });
    if cache.is_some() {
        println!("cache: {cached_games} games replayed from the cache, {stale} stale records replaced");
    }
    println!("{ok}/{} games identical ({ended} ended, {budget} skipped at the core's script budget, the others reached --max-decisions), {decisions} decisions; core {:.3}s, rust {:.3}s", args.games, tc.as_secs_f64(), tr.as_secs_f64());
}
