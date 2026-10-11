#!/usr/bin/env python3
"""Static check: the player/location arguments of a card's Lua calls against its Rust translation.

For calls like Duel.IsExistingMatchingCard(f, p, s, o, ...) the player and the two location
arguments are taken from the Lua and from the Rust (duel::is_existing_matching_card(d, f, p, s, o,
...)), in order of appearance per function, and compared after normalizing (`a.tp`/`tp` -> tp,
`1 - a.tp` -> 1-tp, `LOCATION_X as u32` -> LOCATION_X, ...). It prints the calls that differ: a
literal player where the Lua has tp, swapped locations, ... (heuristic: helper functions or
reordered code can give false positives).

  tools/argcheck.py [CODE ...]     # default: all translated cards
"""
import os
import re
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
LUA = os.path.join(ROOT, "external", "CardScripts", "official")
EXT = os.path.join(ROOT, "core_ext", "lua", "cards")
RS = os.path.join(ROOT, "rust", "ygo-cards", "src", "cards")

# Lua name -> (Rust name, index of the player argument in the Lua call, in the Rust call after `d`)
FUNCS = {
    "IsExistingMatchingCard": ("is_existing_matching_card", 1, 1),
    "IsExistingTarget": ("is_existing_target", 1, 1),
    "SelectMatchingCard": ("select_matching_card", 2, 2),
    "SelectTarget": ("select_target", 2, 2),
    "GetMatchingGroup": ("get_matching_group", 1, 1),
    "GetMatchingGroupCount": ("get_matching_group_count", 1, 1),
    "GetFieldGroup": ("get_field_group", 0, 0),
    "GetFieldGroupCount": ("get_field_group_count", 0, 0),
}


def split_args(s, start):
    """The top-level comma-separated arguments of the call whose '(' is at s[start]."""
    depth = 0
    args, cur = [], ""
    i = start
    while i < len(s):
        ch = s[i]
        if ch in "([{":
            depth += 1
            if depth == 1:
                i += 1
                continue
        elif ch in ")]}":
            depth -= 1
            if depth == 0:
                args.append(cur.strip())
                return args
        elif ch == "," and depth == 1:
            args.append(cur.strip())
            cur = ""
            i += 1
            continue
        if depth >= 1:
            cur += ch
        i += 1
    return args


def norm(a):
    a = re.sub(r"\s+", "", a)
    a = a.replace("a.tp", "tp").replace("a.ep", "ep").replace("a.rp", "rp")
    a = re.sub(r"as(u8|u32|u16|i32|i64|u64)", "", a)
    a = a.replace("(", "").replace(")", "")
    a = a.replace("+", "|")
    a = re.sub(r"Some", "", a)
    return a


def calls(text, name, rust):
    out = []
    pat = (r"\bduel::" if rust else r"\bDuel\.") + re.escape(name) + r"\s*\("
    for m in re.finditer(pat, text):
        args = split_args(text, m.end() - 1)
        out.append(args)
    return out


def check(code):
    lua_path = os.path.join(EXT, f"c{code}.lua")
    if not os.path.exists(lua_path):
        lua_path = os.path.join(LUA, f"c{code}.lua")
    rs_path = os.path.join(RS, f"c{code}.rs")
    if not os.path.exists(lua_path) or not os.path.exists(rs_path):
        return []
    lua = open(lua_path, errors="ignore").read()
    lua = re.sub(r"--[^\n]*", "", lua)
    rs = open(rs_path).read()
    rs = re.sub(r"//[^\n]*", "", rs)
    bad = []
    for lname, (rname, lp, rp) in FUNCS.items():
        lc = calls(lua, lname, False)
        rc = calls(rs, rname, True)
        if len(lc) != len(rc):
            continue
        for i, (la, ra) in enumerate(zip(lc, rc)):
            ra = ra[1:]  # drop `d`
            if len(la) < lp + 3 or len(ra) < rp + 3:
                continue
            l3 = [norm(x) for x in la[lp:lp + 3]]
            r3 = [norm(x) for x in ra[rp:rp + 3]]
            if l3 != r3:
                bad.append(f"{code} {lname}#{i}: lua {la[lp:lp + 3]} rust {ra[rp:rp + 3]}")
    return bad


def main():
    codes = sys.argv[1:] or sorted(f[1:-3] for f in os.listdir(RS) if re.fullmatch(r"c\d+\.rs", f))
    n = 0
    for c in codes:
        for b in check(c):
            print(b)
            n += 1
    print(f"({n} differing calls)", file=sys.stderr)


if __name__ == "__main__":
    main()
