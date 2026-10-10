#!/usr/bin/env python3
"""Which card scripts use only what the Rust library already has.

For each official script: the library names (aux.X, Cost.X, Xyz.X, ...) and API calls
(Duel.X, Card/Group/Effect methods) without a Rust function of the same snake_case name.

  tools/translatable.py                 # summary: the most needed missing names
  tools/translatable.py --ready N       # N cards (most used in decks first) with nothing missing
  tools/translatable.py --card CODE     # what one card misses
"""
import collections
import os
import re
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
SCRIPTS = os.path.join(ROOT, "external", "CardScripts", "official")
RUST = [os.path.join(ROOT, "rust", "ygo-cards", "src"), os.path.join(ROOT, "rust", "ygo-core", "src", "api")]
FREQ = os.path.join(ROOT, "download_human_decks", "card_deck_frequency.tsv")

# Lua-only helpers that translate to closures or plain Rust (see TRANSLATING.md)
BUILTIN = {"aux.Next", "aux.TRUE", "aux.FALSE", "aux.NOT", "aux.AND", "aux.OR", "aux.FilterBoolFunction",
           "aux.FilterBoolFunctionEx", "aux.TargetBoolFunction", "aux.FilterEqualFunction", "aux.FaceupFilter",
           "aux.TargetEqualFunction", "aux.NULL", "aux.Stringid", "Group.CreateGroup", "Group.FromCards", "Effect.CreateEffect",
           "Effect.GlobalEffect"}
# Lua names whose Rust function has another name; None: no function needed (see TRANSLATING.md)
ALIAS = {"SendtoHand": "send_to_hand", "SendtoGrave": "send_to_grave", "SendtoDeck": "send_to_deck",
         "SendtoExtraP": "send_to_extra_p", "Sendto": "send_to", "SetSPSummonOnce": "set_spsummon_once",
         "Merge": "add_card", "Sub": "remove_card", "Match": "match_", "GetChainData": "effect_data",
         "AnnounceLevel": "announce_number_range", "bfgcost": "self_banish", "KeepAlive": None, "DeleteGroup": None, "GetNext": None,
         "Iter": None, "Clear": None}
LIB = r"\b(aux|Auxiliary|Xyz|Synchro|Link|Fusion|Ritual|Pendulum|Gemini|Spirit|Union|Cost|Chain|Equip|Maximum|Duel|Card|Group|Effect)\.([A-Za-z_]\w*)"
METHOD = r":([A-Z]\w*)\s*\("


def snake(n):
    n = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", n)
    n = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", n)
    return n.lower()


def rust_names():
    names = set()
    for d in RUST:
        for root, _, fs in os.walk(d):
            for f in fs:
                if f.endswith(".rs"):
                    s = open(os.path.join(root, f)).read()
                    names.update(re.findall(r"\bfn ([a-z_][a-z0-9_]*)", s))
                    # functions defined by macros: `name, STATIC,`
                    names.update(re.findall(r"^\s*([a-z_][a-z0-9_]*), [A-Z_]+,\s*$", s, re.M))
    return names


def missing_of(code, have):
    path = os.path.join(SCRIPTS, f"c{code}.lua")
    s = open(path, errors="ignore").read()
    s = re.sub(r"--\[\[.*?\]\]", "", s, flags=re.S)
    s = re.sub(r"--[^\n]*", "", s)
    own = set(re.findall(r"function s\.(\w+)", s)) | set(re.findall(r"\bs\.(\w+)\s*=", s))
    out = set()
    for ns, name in re.findall(LIB, s):
        ns = "aux" if ns == "Auxiliary" else ns
        full = f"{ns}.{name}"
        if full in BUILTIN or name.isupper():
            continue
        if name in ALIAS:
            if ALIAS[name] is None or ALIAS[name] in have:
                continue
            out.add(full)
        elif snake(name) not in have:
            out.add(full)
    for name in re.findall(METHOD, s):
        if name in own:
            continue
        if name in ALIAS:
            if ALIAS[name] is None or ALIAS[name] in have:
                continue
            out.add(":" + name)
        elif snake(name) not in have:
            out.add(":" + name)
    return out


def main():
    have = rust_names()
    freq = {}
    if os.path.exists(FREQ):
        for line in open(FREQ).read().splitlines()[1:]:
            cid, _, n = line.split("\t")
            freq[int(cid)] = int(n)
    codes = sorted(int(f[1:-4]) for f in os.listdir(SCRIPTS) if re.fullmatch(r"c\d+\.lua", f))
    done = set(int(f[1:-3]) for f in os.listdir(os.path.join(ROOT, "rust", "ygo-cards", "src", "cards")) if re.fullmatch(r"c\d+\.rs", f))
    if "--card" in sys.argv:
        code = int(sys.argv[sys.argv.index("--card") + 1])
        print(" ".join(sorted(missing_of(code, have))) or "nothing missing")
        return
    miss = {c: missing_of(c, have) for c in codes}
    if "--ready" in sys.argv:
        n = int(sys.argv[sys.argv.index("--ready") + 1])
        ready = [c for c in codes if not miss[c] and c not in done]
        ready.sort(key=lambda c: -freq.get(c, 0))
        for c in ready[:n]:
            print(c, freq.get(c, 0))
        print(f"({len(ready)} cards ready, {len(done)} translated)", file=sys.stderr)
        return
    cnt = collections.Counter()
    weight = collections.Counter()
    for c, m in miss.items():
        for x in m:
            cnt[x] += 1
            weight[x] += freq.get(c, 0)
    ready = sum(1 for c in codes if not miss[c])
    print(f"{len(codes)} scripts, {ready} with nothing missing, {len(done)} translated")
    print("most needed (cards, deck slots):")
    for x, v in sorted(cnt.items(), key=lambda kv: -weight[kv[0]])[:60]:
        print(f"  {v:6d} {weight[x]:9d}  {x}")


main()
