#!/usr/bin/env python3
"""Plan card translation by deck pairs (the opening book's matchups).

Greedily picks the deck pairs that need the fewest untranslated cards, assigns each new card to
the first pair that needs it, and writes the pairs' decks as .ydk files to build/deck_pairs/.

  tools/deck_plan.py --pairs N        # plan the next N pairs: print them and their new cards
  tools/deck_plan.py --status         # how many book pairs are fully translated
"""
import glob
import json
import os
import re
import sqlite3
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
BOOK = os.path.join(ROOT, "runs", "books", "match1.jsonl")
CARDS = os.path.join(ROOT, "rust", "ygo-cards", "src", "cards")
SCRIPTS = os.path.join(ROOT, "external", "CardScripts", "official")
OUT = os.path.join(ROOT, "build", "deck_pairs")

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


def load_alias():
    alias = {}
    for f in glob.glob(os.path.join(ROOT, "external", "BabelCDB", "*.cdb")):
        con = sqlite3.connect(f)
        try:
            for cid, a in con.execute("select id, alias from datas"):
                alias[cid] = a
        except sqlite3.Error:
            pass
    return alias


def script_code(code, alias):
    a = alias.get(code, 0)
    return a if a and a < code + 10 and code < a + 10 else code


def main():
    alias = load_alias()
    translated = set(int(f[1:-3]) for f in os.listdir(CARDS) if re.fullmatch(r"c\d+\.rs", f))
    pairs = []
    seen = set()
    for line in open(BOOK):
        o = json.loads(line)
        a, b = o["decks"]
        key = tuple(sorted([a["name"], b["name"]]))
        if key in seen:
            continue
        seen.add(key)
        need = set()
        for d in (a, b):
            for c in d["main"] + d["extra"]:
                s = script_code(c, alias)
                if os.path.exists(os.path.join(SCRIPTS, f"c{s}.lua")) and s not in translated:
                    need.add(s)
        pairs.append((o, need))
    if "--status" in sys.argv:
        done = sum(1 for _, n in pairs if not n)
        print(f"{len(pairs)} book pairs, {done} fully translated; {len(translated)} cards translated")
        return
    n = int(sys.argv[sys.argv.index("--pairs") + 1]) if "--pairs" in sys.argv else 10
    import translatable
    have = translatable.rust_names()
    covered = set()
    chosen = []
    remaining = list(pairs)
    for _ in range(n):
        if not remaining:
            break
        remaining.sort(key=lambda p: len(p[1] - covered))
        o, need = remaining.pop(0)
        new = sorted(need - covered)
        covered |= need
        chosen.append((o, new))
    os.makedirs(OUT, exist_ok=True)
    for i, (o, new) in enumerate(chosen):
        names = []
        for j, d in enumerate(o["decks"]):
            path = os.path.join(OUT, f"pair{i:03d}_{'ab'[j]}.ydk")
            with open(path, "w") as f:
                f.write("#main\n" + "\n".join(map(str, d["main"])) + "\n#extra\n" + "\n".join(map(str, d["extra"])) + "\n!side\n")
            names.append(d["name"])
        print(f"pair{i:03d}: {len(new)} new cards  [{names[0]}] vs [{names[1]}]")
        for c in new:
            miss = translatable.missing_of(c, have)
            lua = os.path.join(SCRIPTS, f"c{c}.lua")
            name = open(lua, errors="ignore").read().split("\n")[1].lstrip("-").strip()
            print(f"  {c} {name}" + (f"   MISSING {' '.join(sorted(miss))}" if miss else ""))


main()
