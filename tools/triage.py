#!/usr/bin/env python3
"""Rank the cards behind the coverage mismatches (build/coverage_fails/mismatch_*.txt).

A mismatch report shows the first differing message of the core and of the Rust engine. The card
codes in those messages (4-byte little-endian values that are cards of the two decks) are the
suspects; cards that show up in many mismatches come first.

  tools/triage.py [--top N] [--card CODE]   # --card: the mismatch files that name CODE
"""
import collections
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import deck_plan  # noqa: E402

ROOT = deck_plan.ROOT
FAILS = os.path.join(ROOT, "build", "coverage_fails")


def deck_codes(path):
    return set(int(l) for l in open(path) if l.strip().isdigit())


def codes_in(hexline, deck):
    b = bytes.fromhex(hexline.replace(" ", ""))
    found = set()
    for i in range(len(b) - 3):
        v = int.from_bytes(b[i:i + 4], "little")
        if v in deck:
            found.add(v)
    return found


def suspects(path):
    lines = open(path, errors="ignore").read().splitlines()
    m = re.search(r"--decks (\S+),(\S+)", lines[0])
    deck = deck_codes(m[1]) | deck_codes(m[2])
    found = set()
    last = set()
    for l in lines:
        mm = re.match(r"\s*(core|rust)(?:\[\d+\.\.\])?:\s+([0-9a-f ]+)$", l)
        if mm:
            found |= codes_in(mm[2], deck)
        mm = re.match(r"\s*(before|same):\s+([0-9a-f ]+)$", l)
        if mm and codes_in(mm[2], deck):
            last = codes_in(mm[2], deck)
    # no card in the differing messages: the cards of the last message before them that names one
    return found or last


def main():
    alias = deck_plan.load_alias()
    votes = collections.Counter()
    files = collections.defaultdict(list)
    none = 0
    for f in sorted(os.listdir(FAILS)):
        if not f.startswith("mismatch_"):
            continue
        s = {deck_plan.script_code(c, alias) for c in suspects(os.path.join(FAILS, f))}
        if not s:
            none += 1
        for c in s:
            votes[c] += 1
            files[c].append(f)
    if "--card" in sys.argv:
        c = int(sys.argv[sys.argv.index("--card") + 1])
        print("\n".join(files[c]))
        return
    top = int(sys.argv[sys.argv.index("--top") + 1]) if "--top" in sys.argv else 40
    for c, n in votes.most_common(top):
        lua = os.path.join(ROOT, "external", "CardScripts", "official", f"c{c}.lua")
        name = open(lua, errors="ignore").read().split("\n")[1].lstrip("-").strip() if os.path.exists(lua) else ""
        print(f"{n:4d} {c} {name}")
    print(f"({none} mismatches name no card of the decks)")


if __name__ == "__main__":
    main()
