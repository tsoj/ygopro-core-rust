"""What the tools share: the paths (this repository is a submodule of the project, whose checkout
holds the card data, the opening book and build/) and the card codes of scripts."""
import glob
import os
import sqlite3

ENGINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
ROOT = os.path.join(ENGINE, "..")
BOOK = os.path.join(ROOT, "runs", "books", "match1.jsonl")
CARDS = os.path.join(ENGINE, "ygo-cards", "src", "cards")
YGO_DIFF = os.path.join(ENGINE, "target", "release", "ygo-diff")
# (recorded games as ygo-diff replay files; tools/replay_records.py)
DIFF_REPLAYS = os.path.join(ENGINE, "target", "release", "ygo-diff-replays")
# the directories the core searches, in its order (ygo-diff core_ffi::find_script)
SCRIPT_DIRS = [os.path.join(ROOT, "external", "CardScripts", d) for d in ("official", "pre-release", "pre-errata", "goat")]


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
    """The code whose script `code` uses (an alternate artwork's: the original's)."""
    a = alias.get(code, 0)
    return a if a and a < code + 10 and code < a + 10 else code


def script_path(code):
    """The Lua script the core loads for `code` (None: no script)."""
    for d in SCRIPT_DIRS:
        p = os.path.join(d, f"c{code}.lua")
        if os.path.exists(p):
            return p
    return None
