#!/usr/bin/env python3
"""Replay recorded games of the Python agents (heuristic, alphabeta search, policy, ...) on both
engines with ygo-diff --replay: their decisions reach game states random play rarely does.

  tools/replay_records.py RECORDS.jsonl [more.jsonl ...] [--jobs N] [--every K] [--first N] [--timeout S]

Memory: about 0.3 GB per worker; run large sets inside a capped scope anyway, e.g.
  systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0 tools/replay_records.py ...

Only games whose decks are fully translated (by a snapshot of target/release/ygo-diff) are played.
Failures go to build/replay_fails/{mismatch,crash,timeout}_<file>_<line>.txt (line 1: the
command, then the replay file and ygo-diff's output); the replay files to build/replays/.
"""
import concurrent.futures
import json
import multiprocessing
import os
import resource
import shutil
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..", "..")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)
import deck_plan  # noqa: E402
import translatable  # noqa: E402

BIN = os.path.join(ROOT, "build", "replay_bin", "ygo-diff")
FAILS = os.path.join(ROOT, "build", "replay_fails")
REPLAYS = os.path.join(ROOT, "build", "replays")


def limit_memory():
    # (one runaway game must not take the machine's memory: it then fails as a crash)
    resource.setrlimit(resource.RLIMIT_AS, (4 << 30, 4 << 30))


def work(job):
    # (the worker reads its game itself: the parent keeps only offsets, since holding the 1+ GB of
    # records in the parent made every worker hold a copy of them)
    src, i, off, timeout = job
    with open(src) as f:
        f.seek(off)
        line = f.readline()
    from ygoai.game import GameRecord
    import record_to_replay
    name = f"{os.path.splitext(os.path.basename(src))[0]}_{i}"
    path = os.path.join(REPLAYS, name + ".replay")
    try:
        record = GameRecord.from_json(json.loads(line))
        with open(path, "w") as g:
            g.write(record_to_replay.convert(record))
    except Exception as e:  # noqa: BLE001
        return name, "convert", repr(e)[:200]
    cmd = [BIN, "--replay", path, "--max-decisions", "100000"]
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, preexec_fn=limit_memory)
        out = p.stdout + p.stderr
        if "1/1 games identical" in out:
            kind = "ok"
        elif "panicked" in out:
            kind = "crash"
        else:
            kind = "mismatch"
    except subprocess.TimeoutExpired:
        out, kind = "", "timeout"
    if kind != "ok":
        with open(os.path.join(FAILS, f"{kind}_{name}.txt"), "w") as f:
            f.write(" ".join(cmd) + "\n" + out)
    return name, kind, ""


def main():
    args = sys.argv[1:]
    opt = lambda k, d: type(d)(args[args.index(k) + 1]) if k in args else d  # noqa: E731
    jobs, every, first, timeout = opt("--jobs", 24), opt("--every", 1), opt("--first", 10**9), opt("--timeout", 300)
    srcs = [a for a in args if a.endswith(".jsonl")]
    os.makedirs(FAILS, exist_ok=True)
    os.makedirs(REPLAYS, exist_ok=True)
    os.makedirs(os.path.dirname(BIN), exist_ok=True)
    shutil.copy2(os.path.join(ROOT, "rust", "target", "release", "ygo-diff"), BIN)
    have = set(int(x) for x in subprocess.run([BIN, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
    alias = deck_plan.load_alias()
    todo = []
    skipped = 0
    for src in srcs:
        with open(src) as f:
            i = -1
            while True:
                off = f.tell()
                line = f.readline()
                if not line:
                    break
                i += 1
                if i % every:
                    continue
                d = json.loads(line)
                if "decks" not in d or "responses" not in d:
                    continue
                codes = {deck_plan.script_code(c, alias) for deck in d["decks"] for part in ("main", "extra") for c in deck.get(part, [])}
                if any(c not in have and translatable.script_path(c) for c in codes):
                    skipped += 1
                    continue
                todo.append((src, i, off, timeout))
                if len(todo) >= first:
                    break
    print(f"{len(todo)} games to replay ({skipped} skipped: untranslated cards)", flush=True)
    stats = {}
    t0 = time.time()
    # (spawned workers: forked ones would share, then copy, the parent's memory)
    with concurrent.futures.ProcessPoolExecutor(jobs, mp_context=multiprocessing.get_context("spawn")) as ex:
        for n, (name, kind, err) in enumerate(ex.map(work, todo, chunksize=4), 1):
            stats[kind] = stats.get(kind, 0) + 1
            if kind == "convert":
                print(f"  {name}: convert failed: {err}", flush=True)
            if n % 200 == 0 or n == len(todo):
                print(f"{n} games in {time.time() - t0:.0f}s: {stats}", flush=True)


if __name__ == "__main__":
    main()
