#!/usr/bin/env python3
"""Generate decks for cards that no (fully translated) human deck plays, or that coverage rarely
reaches: take the human deck whose cards are most similar to the card, and replace the deck's
cards most similar to it with copies of it.

Similarity: TF-IDF cosine over the card texts (effect text, with archetype set codes and
type/race/attribute as extra tokens, so cards of one archetype or of the same kind are close).

  tools/gen_decks.py [--codes a,b,...] [--uncovered] [--per-card N] [--copies K]

--codes: these cards; --uncovered: the translated cards not yet activated or summoned in an
agreeing coverage game (build/coverage.json). Writes build/gen_decks.jsonl (one book-style line
per deck: {"decks": [{"name", "main", "extra"}]}), which tools/coverage.py also draws from.
"""
import collections
import glob
import json
import math
import os
import re
import sqlite3
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..", "..")
sys.path.insert(0, HERE)
import deck_plan  # noqa: E402
import translatable  # noqa: E402

OUT = os.path.join(ROOT, "build", "gen_decks.jsonl")
COV = os.path.join(ROOT, "build", "coverage.json")
TYPE_EXTRA = 0x40 | 0x2000 | 0x800000 | 0x4000000  # fusion, synchro, xyz, link


def load_cards():
    cards = {}
    for db in sorted(glob.glob(os.path.join(ROOT, "external", "BabelCDB", "*.cdb"))):
        con = sqlite3.connect(db)
        for code, ot, setcode, typ, race, attr, name, desc in con.execute(
                "select d.id, d.ot, d.setcode, d.type, d.race, d.attribute, t.name, t.desc from datas d join texts t on d.id = t.id"):
            cards.setdefault(code, (setcode, typ, race, attr, name, desc or ""))
    return cards


def tokens(card):
    setcode, typ, race, attr, name, desc = card
    words = re.findall(r"[a-z]+", desc.lower())
    toks = [w for w in words if len(w) > 2]
    for i in range(4):
        s = (setcode >> (16 * i)) & 0xFFFF
        if s:
            toks += [f"set{s & 0xFFF:x}"] * 4  # (the archetype base; sub-archetypes share it)
    toks += [f"type{typ:x}", f"race{race:x}", f"attr{attr:x}"]
    return toks


def tfidf(cards, codes):
    docs = {c: collections.Counter(tokens(cards[c])) for c in codes if c in cards}
    df = collections.Counter()
    for t in docs.values():
        df.update(t.keys())
    n = len(docs)
    vecs = {}
    for c, t in docs.items():
        v = {w: (1 + math.log(k)) * math.log(n / df[w]) for w, k in t.items()}
        norm = math.sqrt(sum(x * x for x in v.values())) or 1.0
        vecs[c] = {w: x / norm for w, x in v.items()}
    return vecs


def cos(a, b):
    if len(a) > len(b):
        a, b = b, a
    return sum(x * b.get(w, 0.0) for w, x in a.items())


def main():
    args = sys.argv[1:]
    per_card = int(args[args.index("--per-card") + 1]) if "--per-card" in args else 2
    copies = int(args[args.index("--copies") + 1]) if "--copies" in args else 3
    binary = os.path.join(ROOT, "rust", "target", "release", "ygo-diff")
    done = set(int(x) for x in subprocess.run([binary, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
    alias = deck_plan.load_alias()
    targets = set()
    if "--codes" in args:
        targets |= {int(x) for x in args[args.index("--codes") + 1].split(",")}
    if "--uncovered" in args:
        cov = json.load(open(COV)) if os.path.exists(COV) else {}
        for c in done:
            e = cov.get(str(c), [0, 0, 0, 0])
            if e[1] + e[2] == 0:
                targets.add(c)
    targets = {c for c in targets if c in done}
    # the fully translated human decks (book)
    decks = {}
    for line in open(deck_plan.BOOK):
        for d in json.loads(line)["decks"]:
            if d["name"] in decks:
                continue
            if all(deck_plan.script_code(c, alias) in done or not translatable.script_path(deck_plan.script_code(c, alias)) for c in d["main"] + d["extra"]):
                decks[d["name"]] = d
    cards = load_cards()
    in_decks = {c for d in decks.values() for c in d["main"] + d["extra"]}
    vecs = tfidf(cards, in_decks | targets)
    by_card = collections.defaultdict(set)
    for name, d in decks.items():
        for c in set(d["main"] + d["extra"]):
            by_card[c].add(name)
    print(f"{len(targets)} target cards, {len(decks)} translated human decks", flush=True)
    pool = sorted(c for c in in_decks if c in vecs)
    out = []
    for u in sorted(targets):
        if u not in vecs or u not in cards:
            continue
        extra = bool(cards[u][1] & TYPE_EXTRA)
        sims = sorted(((cos(vecs[u], vecs[c]), c) for c in pool if c != u), reverse=True)[:40]
        simd = dict((c, s) for s, c in sims)
        cand = collections.Counter()
        for s, c in sims:
            for name in by_card[c]:
                cand[name] += s
        for name, _ in cand.most_common(per_card):
            d = decks[name]
            part = "extra" if extra else "main"
            lst = list(d[part])
            # replace the cards most similar to u (not u itself) by copies of u
            order = sorted(range(len(lst)), key=lambda i: -simd.get(lst[i], cos(vecs[u], vecs.get(lst[i], {}))))
            k = min(copies, len(lst))
            if k == 0:
                continue
            for i in order[:k]:
                lst[i] = u
            nd = dict(d, name=f"gen_{u}_{name}")
            nd[part] = lst
            out.append({"decks": [nd]})
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w") as f:
        for o in out:
            f.write(json.dumps(o) + "\n")
    print(f"{len(out)} decks -> {OUT}")


if __name__ == "__main__":
    main()
