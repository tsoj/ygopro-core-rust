#!/usr/bin/env python3
"""The script codes of the cards in .ydk files (for YGO_CARDS_ONLY), space-separated."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

alias = common.load_alias()
codes = set()
args = sys.argv[1:]
if args and args[0] == "--replay":  # a ygo-diff --replay file
    for l in open(args[1]):
        if l.startswith("card "):
            c = int(l.split()[3])
            codes.add(c)
            codes.add(common.script_code(c, alias))
    args = []
for p in args:
    for l in open(p):
        if l.strip().isdigit():
            c = int(l)
            codes.add(c)
            codes.add(common.script_code(c, alias))
print(" ".join(map(str, sorted(codes))))
