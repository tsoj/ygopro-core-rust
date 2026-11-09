#!/usr/bin/env python3
"""Writes the arbiter's reveal table (ygo-arbiter/data/reveals.tsv, see ygo-arbiter/src/reveal.rs)
from the rows of ygo-reveal-audit.

usage: tools/reveal_table.py AUDIT.jsonl... [--merge] > ygo-arbiter/data/reveals.tsv

--merge: keep the lines of the current table (ygo-arbiter/data/reveals.tsv) that the new rows
do not decide (to add the results of a discovery run to the table).

Per selection (effect and move, the audit's "key"):
  * a row with a requirement: "script" if the script showed the card in each such row, else
    "reveal" (the arbiter shows it);
  * else a free row: "hide";
  * else (only undecided checks): as a requirement.
A Normal Summon or Set to the Monster Zone is "hide" (only the rules limit the card).
The "card" lines are the cards of the audited games' decks.
"""
import json
import os
import sys
from collections import defaultdict

MZONE, SZONE = 0x4, 0x8
REASON_SPSUMMON = 0x800
TABLE = os.path.join(os.path.dirname(__file__), '..', 'ygo-arbiter', 'data', 'reveals.tsv')


def main():
    args = [a for a in sys.argv[1:] if a != '--merge']
    merge = '--merge' in sys.argv[1:]
    cards = set()
    rows = defaultdict(list)
    for path in args:
        with open(path) as f:
            for line in f:
                r = json.loads(line)
                if 'deck_cards' in r:
                    cards.update(r['deck_cards'])
                elif 'key' in r and r['verdict'] in ('requirement', 'free', 'undecided'):
                    key = list(r['key'])
                    if key[4] & (MZONE | SZONE):
                        key[5] = False  # (the arbiter's key: a move to the field has no cost)
                    rows[tuple(key)].append(r)
    sel = {}
    for key, rs in rows.items():
        if key[4] == MZONE and all(r['reason'] & REASON_SPSUMMON == 0 for r in rs):
            # a Normal Summon or Set (e.g. by a Tribute Summon procedure): only the rules limit it
            sel[key] = 'hide'
            continue
        req = [r for r in rs if r['verdict'] == 'requirement']
        free = [r for r in rs if r['verdict'] == 'free']
        if not req and not free:
            req = rs
        if req:
            sel[key] = 'script' if all(r['script_confirms'] for r in req) else 'reveal'
        else:
            sel[key] = 'hide'
    lines = {}
    if merge:
        with open(TABLE) as f:
            for line in f:
                fields = line.rstrip('\n').split('\t')
                if fields[0] == 'card':
                    cards.add(int(fields[1]))
                elif fields[0] == 'sel':
                    lines[tuple(fields[1:7])] = fields[7]
    for (card, desc, typ, frm, to, cost), d in sel.items():
        lines[tuple(str(x) for x in (card, desc, typ, frm, to, 'cost' if cost else 'effect'))] = d
    out = sys.stdout
    out.write('# ygo-arbiter reveal table (see src/reveal.rs), written by tools/reveal_table.py\n')
    out.write('# card CODE: a card of the audited decks\n')
    out.write('# sel CARD DESC TYPE FROM TO cost|effect DECISION: the selection of an effect '
              '(owner\'s code, description, type; card 0 = by the rules) that moves a card '
              'from FROM to TO (locations); DECISION reveal, script or hide\n')
    for c in sorted(cards):
        out.write(f'card\t{c}\n')
    for k in sorted(lines, key=lambda k: tuple(int(x) if x.isdigit() else x for x in k)):
        out.write('sel\t' + '\t'.join(k) + '\t' + lines[k] + '\n')
    print(f'{len(cards)} cards, {len(lines)} selections: '
          + ', '.join(f'{d} {sum(v == d for v in lines.values())}' for d in ('reveal', 'script', 'hide')),
          file=sys.stderr)


if __name__ == '__main__':
    main()
