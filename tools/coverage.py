#!/usr/bin/env python3
"""Card coverage: single games between human decks, Rust engine vs the core (ygo-diff).

Each game takes two decks of the opening book whose cards are all translated, chosen for the
translated cards that were played least so far, and plays one game with a fresh seed. A game where
both engines agree adds its cards' activations (MSG_CHAINING) and summons to build/coverage.json;
a mismatch is saved to build/coverage_fails/ with its decks and seed (to reproduce it with ygo-diff).

  tools/coverage.py run [--jobs 30] [--games N] [--timeout 120]
  tools/coverage.py focus [--games N]    # the deck pairs of build/focus_decks, with --focus
  tools/coverage.py status [--list N]   # coverage of the translated cards (--list: N least played)
  tools/coverage.py import              # add the pair tests' coverage (build/pair_results)
"""
import concurrent.futures
import json
import os
import random
import re
import shutil
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import deck_plan  # noqa: E402
import translatable  # noqa: E402

ROOT = deck_plan.ROOT
COV = os.path.join(ROOT, "build", "coverage.json")
FAILS = os.path.join(ROOT, "build", "coverage_fails")
DECKS = os.path.join(ROOT, "build", "coverage_decks")
BIN = os.path.join(ROOT, "build", "coverage_bin", "ygo-diff")
CARDS = deck_plan.CARDS


def translated():
    return set(int(f[1:-3]) for f in os.listdir(CARDS) if re.fullmatch(r"c\d+\.rs", f))


def load_cov():
    if os.path.exists(COV):
        return {int(k): v for k, v in json.load(open(COV)).items()}
    return {}


def save_cov(cov):
    tmp = COV + ".tmp"
    with open(tmp, "w") as f:
        json.dump({str(k): v for k, v in cov.items()}, f)
    os.replace(tmp, COV)


def entry(cov, c):
    # [games in a deck of an agreeing game, activations, summons, mismatching games]
    return cov.setdefault(c, [0, 0, 0, 0])


def book_decks(alias, done):
    """The book's distinct decks (and the generated ones of build/gen_decks.jsonl) whose cards all
    have a translated script (or need none)."""
    decks = {}
    gen = os.path.join(ROOT, "build", "gen_decks.jsonl")  # tools/gen_decks.py
    lines = list(open(deck_plan.BOOK)) + (list(open(gen)) if os.path.exists(gen) else [])
    for line in lines:
        for d in json.loads(line)["decks"]:
            if d["name"] in decks:
                continue
            cards = set()
            ok = True
            for c in d["main"] + d["extra"]:
                s = deck_plan.script_code(c, alias)
                if translatable.script_path(s):
                    if s not in done:
                        ok = False
                        break
                    cards.add(s)
            if ok:
                decks[d["name"]] = (d, cards)
    return decks


def weight(e):
    games, act, summ = e[0], e[1], e[2]
    return 1.0 / (1.0 + act + summ + 0.25 * games) ** 2


def ydk_path(name, d):
    safe = re.sub(r"[^A-Za-z0-9_.-]", "_", name)[:80]
    p = os.path.join(DECKS, f"{safe}.ydk")
    if not os.path.exists(p):
        with open(p, "w") as f:
            f.write("#main\n" + "\n".join(map(str, d["main"])) + "\n#extra\n" + "\n".join(map(str, d["extra"])) + "\n!side\n")
    return p


COVLINE = re.compile(r"^\s+(\d+): (\d+) activations, (\d+) summons$")


def parse_cov(text, alias):
    out = {}
    for line in text.splitlines():
        m = COVLINE.match(line)
        if m:
            c = deck_plan.script_code(int(m[1]), alias)
            a, s = out.get(c, (0, 0))
            out[c] = (a + int(m[2]), s + int(m[3]))
    return out


def play(pa, pb, seed, timeout, extra=()):
    cmd = [BIN, "--decks", f"{pa},{pb}", "--games", "1", "--seed", str(seed), *extra]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
        return cmd, r.returncode, r.stdout + r.stderr
    except subprocess.TimeoutExpired as e:
        out = (e.stdout or b"").decode() if isinstance(e.stdout, bytes) else (e.stdout or "")
        return cmd, None, out


def run(args):
    jobs = int(args[args.index("--jobs") + 1]) if "--jobs" in args else 30
    games = int(args[args.index("--games") + 1]) if "--games" in args else 10**9
    timeout = int(args[args.index("--timeout") + 1]) if "--timeout" in args else 120
    os.makedirs(FAILS, exist_ok=True)
    os.makedirs(DECKS, exist_ok=True)
    os.makedirs(os.path.dirname(BIN), exist_ok=True)
    # a snapshot of the binary: rebuilds during the run do not change it
    shutil.copy2(os.path.join(ROOT, "rust", "target", "release", "ygo-diff"), BIN)
    alias = deck_plan.load_alias()
    # the cards of the snapshot (the source tree can have newer ones)
    done = set(int(x) for x in subprocess.run([BIN, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
    decks = book_decks(alias, done)
    names = list(decks)
    print(f"{len(names)} book decks with all cards translated; {len(done)} cards translated", flush=True)
    cov = load_cov()
    pending = {}  # card -> games in flight (counted as played for the choice)
    rng = random.Random()

    def score(name):
        s = 0.0
        for c in decks[name][1]:
            e = list(cov.get(c, [0, 0, 0, 0]))
            e[0] += pending.get(c, 0)
            s += weight(e)
        return s

    def pick(exclude=None):
        cand = rng.sample(names, min(400, len(names)))
        cand = [n for n in cand if n != exclude]
        cand.sort(key=score, reverse=True)
        return rng.choice(cand[:5])

    stats = {"ok": 0, "mismatch": 0, "timeout": 0, "crash": 0, "budget": 0}
    started = 0
    t0 = time.time()
    with concurrent.futures.ThreadPoolExecutor(jobs) as ex:
        running = {}

        def submit():
            nonlocal started
            a = pick()
            b = pick(a)
            for c in decks[a][1] | decks[b][1]:
                pending[c] = pending.get(c, 0) + 1
            seed = rng.getrandbits(63)
            f = ex.submit(play, ydk_path(a, decks[a][0]), ydk_path(b, decks[b][0]), seed, timeout)
            running[f] = (a, b, seed)
            started += 1

        while started < games and len(running) < jobs:
            submit()
        while running:
            fin, _ = concurrent.futures.wait(list(running), return_when=concurrent.futures.FIRST_COMPLETED)
            for f in fin:
                a, b, seed = running.pop(f)
                cards = decks[a][1] | decks[b][1]
                for c in cards:
                    pending[c] -= 1
                cmd, code, out = f.result()
                if code is None:
                    stats["timeout"] += 1
                    kind = "timeout"
                elif "MISMATCH" in out:
                    stats["mismatch"] += 1
                    kind = "mismatch"
                elif "1/1 games identical" in out:
                    kind = "ok"
                    stats["ok"] += 1
                    for c in cards:
                        entry(cov, c)[0] += 1
                    for c, (act, summ) in parse_cov(out, alias).items():
                        e = entry(cov, c)
                        e[1] += act
                        e[2] += summ
                elif "skipped at the core's script budget" in out and "0/1" in out:
                    stats["budget"] += 1
                    kind = "budget"
                else:
                    stats["crash"] += 1
                    kind = "crash"
                if kind in ("mismatch", "timeout", "crash"):
                    if kind == "mismatch":
                        for c in cards:
                            entry(cov, c)[3] += 1
                    with open(os.path.join(FAILS, f"{kind}_{seed}.txt"), "w") as fo:
                        fo.write(" ".join(cmd) + "\n" + out[-20000:])
                if started < games:
                    submit()
            n = sum(stats.values())
            if n % 50 == 0 or not running:
                save_cov(cov)
                played = sum(1 for c in done if cov.get(c, [0, 0, 0])[1] + cov.get(c, [0, 0, 0])[2] > 0)
                print(f"{n} games in {time.time() - t0:.0f}s: {stats}; {played}/{len(done)} translated cards activated or summoned", flush=True)
    save_cov(cov)


FOCUS = os.path.join(ROOT, "build", "focus_decks")


def focus(args):
    """The deck pairs of build/focus_decks (<CODE>_a.ydk, <CODE>_b.ydk; written by the focus agents,
    see rust/ygo-cards/FOCUS.md), each played with `--focus CODE` (the player uses that card whenever
    it can): cards random play never reaches."""
    jobs = int(args[args.index("--jobs") + 1]) if "--jobs" in args else 24
    per = int(args[args.index("--games") + 1]) if "--games" in args else 10
    timeout = int(args[args.index("--timeout") + 1]) if "--timeout" in args else 120
    os.makedirs(FAILS, exist_ok=True)
    os.makedirs(os.path.dirname(BIN), exist_ok=True)
    shutil.copy2(os.path.join(ROOT, "rust", "target", "release", "ygo-diff"), BIN)
    alias = deck_plan.load_alias()
    done = set(int(x) for x in subprocess.run([BIN, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
    pairs = []
    for f in sorted(os.listdir(FOCUS)):
        m = re.fullmatch(r"(\d+)_a\.ydk", f)
        if m and os.path.exists(os.path.join(FOCUS, f"{m[1]}_b.ydk")):
            pa, pb = os.path.join(FOCUS, f), os.path.join(FOCUS, f"{m[1]}_b.ydk")
            cards = set()
            ok = True
            for p in (pa, pb):
                for l in open(p):
                    if l.strip().isdigit():
                        c = deck_plan.script_code(int(l), alias)
                        if translatable.script_path(c):
                            if c not in done:
                                ok = False
                            cards.add(c)
            if ok:
                pairs.append((int(m[1]), pa, pb, cards))
    print(f"{len(pairs)} focus deck pairs, {per} games each", flush=True)
    cov = load_cov()
    rng = random.Random()
    stats = {"ok": 0, "mismatch": 0, "timeout": 0, "crash": 0, "budget": 0}
    todo = [(code, pa, pb, cards, rng.getrandbits(63)) for code, pa, pb, cards in pairs for _ in range(per)]
    with concurrent.futures.ThreadPoolExecutor(jobs) as ex:
        futs = {ex.submit(play, pa, pb, seed, timeout, ("--focus", str(code))): (code, cards, seed) for code, pa, pb, cards, seed in todo}
        for f in concurrent.futures.as_completed(futs):
            code, cards, seed = futs[f]
            cmd, rc, out = f.result()
            if rc is None:
                kind = "timeout"
            elif "MISMATCH" in out:
                kind = "mismatch"
            elif "1/1 games identical" in out:
                kind = "ok"
                for c in cards:
                    entry(cov, c)[0] += 1
                for c, (act, summ) in parse_cov(out, alias).items():
                    e = entry(cov, c)
                    e[1] += act
                    e[2] += summ
            elif "skipped at the core's script budget" in out and "0/1" in out:
                kind = "budget"
            else:
                kind = "crash"
            stats[kind] += 1
            if kind in ("mismatch", "timeout", "crash"):
                with open(os.path.join(FAILS, f"{kind}_focus{code}_{seed}.txt"), "w") as fo:
                    fo.write(" ".join(cmd) + "\n" + out[-20000:])
    save_cov(cov)
    used = sum(1 for code, *_ in pairs if sum(cov.get(code, [0, 0, 0, 0])[1:3]) > 0)
    print(f"{sum(stats.values())} games: {stats}; {used}/{len(pairs)} focus cards activated or summoned", flush=True)


def status(args):
    cov = load_cov()
    done = translated()
    n = len(done)
    used = [c for c in done if sum(cov.get(c, [0, 0, 0])[1:3]) > 0]
    in_game = [c for c in done if cov.get(c, [0])[0] > 0]
    ge5 = [c for c in done if sum(cov.get(c, [0, 0, 0])[1:3]) >= 5]
    bad = [c for c in done if len(cov.get(c, [])) > 3 and cov[c][3] > 0]
    print(f"{n} translated cards: {len(in_game)} in a deck of an agreeing game, {len(used)} activated or summoned (>=5 times: {len(ge5)}); {len(bad)} in a deck of a mismatching game")
    if "--list" in args:
        k = int(args[args.index("--list") + 1])
        rest = sorted(done, key=lambda c: (sum(cov.get(c, [0, 0, 0])[1:3]), cov.get(c, [0])[0]))
        for c in rest[:k]:
            print(c, cov.get(c, [0, 0, 0, 0]))


def import_pairs(args):
    alias = deck_plan.load_alias()
    cov = load_cov()
    res = os.path.join(ROOT, "build", "pair_results")
    n = 0
    for f in sorted(os.listdir(res)):
        text = open(os.path.join(res, f), errors="ignore").read()
        m = re.search(r"^(\d+)/(\d+) games identical", text, re.M)
        if not m or m[1] != m[2]:
            continue
        games = int(m[1])
        pc = parse_cov(text, alias)
        for c, (act, summ) in pc.items():
            e = entry(cov, c)
            e[0] += games
            e[1] += act
            e[2] += summ
        n += 1
    save_cov(cov)
    print(f"imported {n} pair results")


if __name__ == "__main__":
    {"run": run, "focus": focus, "status": status, "import": import_pairs}[sys.argv[1]](sys.argv[2:])
