#!/usr/bin/env python3
"""Static check: negated predicate calls of a card's Lua against its Rust translation.

Counts, per predicate (`IsX`/`CheckX`/`HasX`), the calls the Lua negates (`not c:IsX(`, `not
Duel.IsX(`) and the calls the Rust negates (`!card::is_x(`, `!duel::is_x(`, `!is_x(`). A card where
the counts differ may have a stray or a missing negation (heuristic: rewritten conditions such as
`if not a then return end` -> `if a { ... }` give many false positives: of ~900 candidates the
three checked were all restructured guards, so use it on one card, not as a sweep).

  tools/negcheck.py [CODE ...]     # default: all translated cards
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from argcheck import EXT, LUA, RS  # noqa: E402

PRED = r"((?:Is|Check|Has)[A-Z][A-Za-z]*)"


def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def counts(code):
    lp = os.path.join(EXT, f"c{code}.lua")
    if not os.path.exists(lp):
        lp = os.path.join(LUA, f"c{code}.lua")
    rp = os.path.join(RS, f"c{code}.rs")
    if not os.path.exists(lp) or not os.path.exists(rp):
        return {}
    lua = re.sub(r"--[^\n]*", "", open(lp, errors="ignore").read())
    rs = re.sub(r"//[^\n]*", "", open(rp).read())
    res = {}
    for m in re.finditer(r"[:.]" + PRED + r"\s*\(", lua):
        name = m[1]
        if name in res:
            continue
        sn = snake(name)
        lt = len(re.findall(r"[:.]" + name + r"\s*\(", lua))
        ln = len(re.findall(r"\bnot\s+[\w.:\[\]()]*?[:.]" + name + r"\s*\(", lua))
        rt = len(re.findall(r"\b" + sn + r"\s*\(", rs))
        rn = len(re.findall(r"!\s*(?:[a-z_]+::)*" + sn + r"\s*\(", rs))
        if lt == rt and ln != rn:
            res[name] = (ln, rn, lt)
    return res


def main():
    codes = sys.argv[1:] or sorted(f[1:-3] for f in os.listdir(RS) if re.fullmatch(r"c\d+\.rs", f))
    n = 0
    for c in codes:
        for name, (ln, rn, t) in counts(c).items():
            print(f"{c} {name}: lua negates {ln}/{t}, rust {rn}/{t}")
            n += 1
    print(f"({n} differing predicates)", file=sys.stderr)


if __name__ == "__main__":
    main()
