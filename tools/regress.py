#!/usr/bin/env python3
"""The engine's regression test: the games of regress/suite.txt on the Rust engine alone
(ygo-diff --rust-only --digest), each game's digest (a hash of its messages, statuses and queried
states) against regress/expected.txt.zst.

  tools/regress.py                 # all jobs (~180k games; ~35 min on 28 threads)
  tools/regress.py --quick         # every 20th job and all cases (~2 min)
  tools/regress.py --filter REGEX  # the jobs whose name matches
  tools/regress.py --bless         # write the digests of the jobs run as the expected ones
  options: --bin PATH (default target/release/ygo-diff), --jobs N, --timeout S (per job)

A job is a line of suite.txt: NAME ygo-diff-arguments. Its decks and replays are in
regress/data.tar.zst (unpacked to target/regress/data). A changed digest is printed with the
command that plays that game alone; whether the new game is right is for the rules (the C++ core
with ygo-diff without --rust-only can help, it is not the reference).
"""
import argparse
import concurrent.futures
import hashlib
import os
import re
import subprocess
import sys
import tarfile
from compression import zstd

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

REGRESS = os.path.join(common.ENGINE, "regress")
SUITE = os.path.join(REGRESS, "suite.txt")
EXPECTED = os.path.join(REGRESS, "expected.txt.zst")
DATA = os.path.join(REGRESS, "data.tar.zst")
WORK = os.path.join(common.ENGINE, "target", "regress")
QUICK_EVERY = 20


def read_suite():
    jobs = []
    for line in open(SUITE):
        line = line.strip()
        if line and not line.startswith("#"):
            name, *args = line.split()
            jobs.append((name, args))
    return jobs


def read_expected():
    """name -> list of 'hash:decisions' (hash: the digest's low 32 bits, hex)."""
    if not os.path.exists(EXPECTED):
        return {}
    exp = {}
    for line in zstd.decompress(open(EXPECTED, "rb").read()).decode().splitlines():
        name, *games = line.split()
        exp[name] = games
    return exp


def write_expected(exp):
    text = "".join(f"{n} {' '.join(g)}\n" for n, g in sorted(exp.items()))
    open(EXPECTED, "wb").write(zstd.compress(text.encode(), level=19))


def unpack():
    """The decks and replays in WORK/data (again when data.tar.zst changed)."""
    digest = hashlib.sha256(open(DATA, "rb").read()).hexdigest()
    stamp = os.path.join(WORK, "data.sha256")
    data = os.path.join(WORK, "data")
    if os.path.exists(stamp) and open(stamp).read() == digest:
        return data
    os.makedirs(data, exist_ok=True)
    with zstd.open(DATA, "rb") as f, tarfile.open(fileobj=f, mode="r|") as tar:
        tar.extractall(data, filter="data")
    open(stamp, "w").write(digest)
    return data


def run_job(binary, data, name, args, timeout):
    """('ok' | error text, list of 'hash:decisions')."""
    env = dict(os.environ, YGO_ROOT=os.environ.get("YGO_ROOT", common.ROOT))
    try:
        p = subprocess.run([binary, "--rust-only", "--digest", *args], cwd=data, env=env, capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return f"timeout after {timeout} s", []
    games = []
    for line in p.stdout.splitlines():
        if line.startswith("digest "):
            _, _, h, n, _ = line.split()
            games.append(f"{h[-8:]}:{n}")
    errors = [l for l in p.stderr.splitlines() if re.search(r"panicked|loops without end", l)]
    if p.returncode != 0:
        errors.append(f"exit {p.returncode}")
    return ("; ".join(errors[:3]) or "ok"), games


def game_command(name, args, game):
    """The ygo-diff command that plays game `game` of a job alone."""
    if "--replay" in args:
        return f"ygo-diff --rust-only -v {' '.join(args)}"
    a = list(args)
    first = int(a[a.index("--from") + 1]) if "--from" in a else 0
    if "--from" in a:
        i = a.index("--from")
        del a[i:i + 2]
    i = a.index("--games")
    a[i + 1] = str(first + game + 1)
    return f"ygo-diff --rust-only -v --from {first + game} {' '.join(a)}"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--bin", default=common.YGO_DIFF)
    ap.add_argument("--quick", action="store_true")
    ap.add_argument("--filter")
    ap.add_argument("--bless", action="store_true")
    ap.add_argument("--jobs", type=int, default=os.cpu_count())
    ap.add_argument("--timeout", type=int, default=1800)
    a = ap.parse_args()
    jobs = read_suite()
    if a.quick:
        jobs = [j for i, j in enumerate(jobs) if j[0].startswith("case_") or i % QUICK_EVERY == 0]
    if a.filter:
        jobs = [j for j in jobs if re.search(a.filter, j[0])]
    expected = read_expected()
    data = unpack()
    binary = os.path.abspath(a.bin)
    print(f"{len(jobs)} jobs, {binary}", flush=True)
    results = {}
    with concurrent.futures.ThreadPoolExecutor(a.jobs) as ex:
        futs = {ex.submit(run_job, binary, data, n, args, a.timeout): (n, args) for n, args in jobs}
        for k, fut in enumerate(concurrent.futures.as_completed(futs), 1):
            results[futs[fut][0]] = fut.result()
            if k % 500 == 0:
                print(f"  {k}/{len(jobs)} jobs", flush=True)
    games = sum(len(g) for _, g in results.values())
    bad = set()
    for name, args in jobs:
        status, got = results[name]
        exp = expected.get(name)
        if status != "ok":
            bad.add(name)
            print(f"ERROR {name}: {status}")
        if a.bless:
            continue
        if exp is None:
            bad.add(name)
            print(f"NEW {name}: no expected digests ({len(got)} games)")
        elif got != exp:
            bad.add(name)
            k = next((i for i, (x, y) in enumerate(zip(got, exp)) if x != y), min(len(got), len(exp)))
            print(f"DIFFERS {name}: game {k} of {len(exp)} (now {len(got)} games): {game_command(name, args, k)}")
    if a.bless:
        for name, (status, got) in results.items():
            expected[name] = got
        write_expected(expected)
        print(f"blessed {len(results)} jobs, {games} games")
    print(f"{len(jobs)} jobs, {games} games, {len(bad)} jobs differ or failed")
    sys.exit(1 if bad and not a.bless else 0)


main()
