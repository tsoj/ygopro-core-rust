#!/usr/bin/env python3
"""The script codes of the cards in .ydk files (for YGO_CARDS_ONLY), space-separated."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import deck_plan  # noqa: E402

alias = deck_plan.load_alias()
codes = set()
for p in sys.argv[1:]:
    for l in open(p):
        if l.strip().isdigit():
            c = int(l)
            codes.add(c)
            codes.add(deck_plan.script_code(c, alias))
print(" ".join(map(str, sorted(codes))))
