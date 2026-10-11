#!/usr/bin/env python3
"""Convert recorded games of the Python agents (ygoai GameRecord .jsonl: match, self-play, SPRT
records) to ygo-diff --replay files, so their decisions (heuristic, policy, search) are replayed
on both engines.

The record is replayed on the core with ygoai.game.replay while the duel calls are logged: the
cards in creation order, the order of every pile at the start (by card id), and the raw bytes of
each response.

  tools/record_to_replay.py RECORDS.jsonl OUTDIR [--first N] [--every K] [--only-translated]

Writes OUTDIR/<line>.replay; prints one line per game.
"""
import json
import os
import sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
sys.path.insert(0, ROOT)
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from ygoai import constants as C  # noqa: E402
from ygoai.core import Duel  # noqa: E402
from ygoai.game import GameRecord, replay  # noqa: E402


def convert(record: GameRecord) -> str:
    log = {"cards": [], "orders": [], "responses": []}
    orig_new_card, orig_start, orig_set_response = Duel.new_card, Duel.start, Duel.set_response

    def new_card(self, owner, code, location, controller=None, sequence=0, position=C.POS_FACEDOWN_DEFENSE):
        assert controller in (None, owner) and sequence == 0 and position == C.POS_FACEDOWN_DEFENSE
        log["cards"].append((owner, location, code))
        return orig_new_card(self, owner, code, location, controller, sequence, position)

    def start(self):
        for p in (0, 1):
            for loc in (C.LOCATION_DECK, C.LOCATION_EXTRA):
                ids = [cid for _, cid in self.location_cards(p, loc)]
                if ids:
                    log["orders"].append((p, loc, ids))
        return orig_start(self)

    def set_response(self, data):
        log["responses"].append(bytes(data))
        return orig_set_response(self, data)

    Duel.new_card, Duel.start, Duel.set_response = new_card, start, set_response
    try:
        # (the duel must be closed: the core's registry keeps it alive, so every converted game
        # leaked a whole duel and its Lua state, and replay workers grew to several GB)
        replay(record, []).close()
    finally:
        Duel.new_card, Duel.start, Duel.set_response = orig_new_card, orig_start, orig_set_response
    cfg = record.duel_config()
    lines = ["seed " + " ".join(str(x & 0xFFFFFFFFFFFFFFFF) for x in record.core_seed),
             f"config {cfg.flags} {cfg.start_lp} {cfg.start_hand} {cfg.draw_per_turn}"]
    lines += [f"card {p} {loc} {code}" for p, loc, code in log["cards"]]
    lines += [f"order {p} {loc} " + " ".join(map(str, ids)) for p, loc, ids in log["orders"]]
    lines += ["resp " + r.hex() for r in log["responses"]]
    return "\n".join(lines) + "\n"


def main():
    args = sys.argv[1:]
    src, out = args[0], args[1]
    first = int(args[args.index("--first") + 1]) if "--first" in args else None
    every = int(args[args.index("--every") + 1]) if "--every" in args else 1
    only = None
    if "--only-translated" in args:
        import subprocess
        binary = os.path.join(ROOT, "rust", "target", "release", "ygo-diff")
        only = set(int(x) for x in subprocess.run([binary, "--list-cards"], capture_output=True, text=True, check=True).stdout.split())
        import deck_plan
        import translatable
        alias = deck_plan.load_alias()
    os.makedirs(out, exist_ok=True)
    n = 0
    with open(src) as f:
        for i, line in enumerate(f):
            if first is not None and n >= first:
                break
            if i % every:
                continue
            record = GameRecord.from_json(json.loads(line))
            if only is not None:
                codes = {deck_plan.script_code(c, alias) for d in record.decks for c in d.main + d.extra}
                missing = [c for c in codes if c not in only and translatable.script_path(c)]
                if missing:
                    continue
            path = os.path.join(out, f"{i}.replay")
            with open(path, "w") as g:
                g.write(convert(record))
            print(path, record.agents, record.result.get("reason"), len(record.responses), flush=True)
            n += 1


if __name__ == "__main__":
    main()
