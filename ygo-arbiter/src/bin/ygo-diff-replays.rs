//! Recorded games (ygo-arbiter's records) as `ygo-diff --replay` files (`game::diff_replay`): the
//! bots' decisions reach game states random play rarely does, and ygo-diff plays them on the
//! Rust engine and on the C++ core (tools/replay_records.py).
//!
//!   ygo-diff-replays RECORDS.jsonl OUTDIR [--every K] [--first N] [--jobs J]
//!
//! Writes OUTDIR/<records' stem>_<line>.replay for every K-th game, at most N.

use std::io::BufRead;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use ygo_arbiter::game::{GameRecord, diff_replay, init};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let usage = || -> ! {
        eprintln!("usage: ygo-diff-replays RECORDS.jsonl OUTDIR [--every K] [--first N] [--jobs J]");
        std::process::exit(2)
    };
    if a.len() < 3 || (a.len() - 3) % 2 != 0 {
        usage();
    }
    let (mut every, mut first, mut jobs) = (1, usize::MAX, 8);
    for o in a[3..].chunks(2) {
        let v: usize = o[1].parse().unwrap_or_else(|_| usage());
        match o[0].as_str() {
            "--every" => every = v.max(1),
            "--first" => first = v,
            "--jobs" => jobs = v.max(1),
            _ => usage(),
        }
    }
    init(None).unwrap();
    let stem = Path::new(&a[1]).file_stem().expect("records file").to_string_lossy().to_string();
    let out = Arc::new(Path::new(&a[2]).to_path_buf());
    std::fs::create_dir_all(&*out).expect("output directory");
    // (the games are read as the workers go: a whole records file would not fit in memory)
    let lines = std::io::BufReader::new(std::fs::File::open(&a[1]).expect("records")).lines().enumerate().filter(move |(i, _)| i % every == 0);
    let games = Arc::new(Mutex::new(lines.take(first)));
    let (done, failed) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    let threads: Vec<_> = (0..jobs)
        .map(|_| {
            let (games, out, stem, done, failed) = (games.clone(), out.clone(), stem.clone(), done.clone(), failed.clone());
            std::thread::spawn(move || {
                loop {
                    let Some((i, l)) = games.lock().unwrap().next() else { break };
                    let record: GameRecord = serde_json::from_str(&l.unwrap()).expect("a GameRecord line");
                    match diff_replay(&record) {
                        Ok(text) => {
                            std::fs::write(out.join(format!("{stem}_{i}.replay")), text).expect("replay file");
                            done.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(e) => {
                            eprintln!("game {i}: {}", e.0);
                            failed.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    eprintln!("{} replay files in {} ({} games failed)", done.load(Ordering::Relaxed), out.display(), failed.load(Ordering::Relaxed));
}
