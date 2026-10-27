#!/usr/bin/env python3
"""Replay recorded games of engines (ygo-arbiter's records) on both engines with ygo-diff --replay:
their decisions reach game states random play rarely does.

  tools/replay_records.py RECORDS.jsonl [more.jsonl ...] [--jobs N] [--every K] [--first N] [--timeout S]

The records become replay files in build/replays/ (ygo-diff-replays; --every and --first apply per
records file). Only games whose cards are all translated (by a snapshot of target/release/ygo-diff) are
played. Failures go to build/replay_fails/{mismatch,crash,timeout}_<game>.txt (line 1: the
command, then ygo-diff's output).
"""
import concurrent.futures
import os
import resource
import shutil
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

ROOT = common.ROOT
BIN = os.path.join(ROOT, "build", "replay_bin", "ygo-diff")
FAILS = os.path.join(ROOT, "build", "replay_fails")
REPLAYS = os.path.join(ROOT, "build", "replays")


def limit_memory():
    # (one runaway game must not take the machine's memory: it then fails as a crash)
    resource.setrlimit(resource.RLIMIT_AS, (4 << 30, 4 << 30))


def play(path, timeout):
    name = os.path.splitext(os.path.basename(path))[0]
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
    return kind


def translated(path, have, alias):
    """Whether every card of a replay file has a translated script (or needs none)."""
    for line in open(path):
        if line.startswith("card "):
            c = common.script_code(int(line.split()[3]), alias)
            if c not in have and common.script_path(c):
                return False
    return True


def main():
    args = sys.argv[1:]
    opt = lambda k, d: type(d)(args[args.index(k) + 1]) if k in args else d  # noqa: E731
    jobs, every, first, timeout = opt("--jobs", 24), opt("--every", 1), opt("--first", 10**9), opt("--timeout", 300)
    srcs = [a for a in args if a.endswith(".jsonl")]
    os.makedirs(FAILS, exist_ok=True)
    os.makedirs(REPLAYS, exist_ok=True)
    os.makedirs(os.path.dirname(BIN), exist_ok=True)
    # a snapshot of the binary: rebuilds during the run do not change it
    shutil.copy2(common.YGO_DIFF, BIN)
    have = set(int(x) for x in subprocess.run([BIN, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
    alias = common.load_alias()
    todo = []
    for src in srcs:
        stem = os.path.splitext(os.path.basename(src))[0]
        old = [f for f in os.listdir(REPLAYS) if f.startswith(stem + "_")]
        for f in old:
            os.remove(os.path.join(REPLAYS, f))
        subprocess.run([common.DIFF_REPLAYS, src, REPLAYS, "--every", str(every), "--first", str(first), "--jobs", str(jobs)], check=True)
        todo += sorted(os.path.join(REPLAYS, f) for f in os.listdir(REPLAYS) if f.startswith(stem + "_"))
    played = [p for p in todo if translated(p, have, alias)]
    print(f"{len(played)} games to replay ({len(todo) - len(played)} skipped: untranslated cards)", flush=True)
    stats = {}
    t0 = time.time()
    with concurrent.futures.ThreadPoolExecutor(jobs) as ex:
        for n, kind in enumerate(ex.map(lambda p: play(p, timeout), played), 1):
            stats[kind] = stats.get(kind, 0) + 1
            if n % 200 == 0 or n == len(played):
                print(f"{n} games in {time.time() - t0:.0f}s: {stats}", flush=True)


if __name__ == "__main__":
    main()
