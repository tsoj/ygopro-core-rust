# Fixing a diverging deck pair (instructions for a fix agent)

The Rust port of ygopro-core (with the Lua card scripts translated to Rust) is tested against the
original (libocgcore.so) by `ygo-diff`, which plays random duels on both and compares every message.
Your deck pair diverges: find which card translation is wrong and fix it.

You work in a git worktree. MAIN=/home/tsoj/Dokumente/Projects/yugioh_AI_2 is the main checkout;
your worktree lacks `external/` and `build/` (read them from MAIN: Lua scripts in
MAIN/external/CardScripts/official/c<CODE>.lua and utility.lua/proc_*.lua there, the C++ core in
MAIN/build/ocgcore-src/).

Reproduce (from your worktree, in the foreground; PAIR is e.g. pair039):
```
cd rust && cargo build --release 2>&1 | tail -2
M=/home/tsoj/Dokumente/Projects/yugioh_AI_2
YGO_ROOT=$M ./target/release/ygo-diff --decks $M/build/deck_pairs/PAIR_a.ydk,$M/build/deck_pairs/PAIR_b.ydk --games 300 --seed 1 --cache $M/build/diff-cache 2>&1 | grep -v "games identical$" | grep -v "^  [0-9]*: "
```
It stops at the first mismatch and prints the game, the decision, the last messages
(`before`/`same`) and the differing message (core vs rust, hex). The first byte of a message is its
type (MSG_* in rust/ygo-core/src/constants.rs); card codes are little-endian u32
(`1a ec 86 00` = 0x0086ec1a = 8842266). `--games G+1 -v` with the same seed replays up to that game
with the decisions printed.

1. Decode the differing message and the messages before it: which card was activated or resolving,
   what the core did that Rust did not (or the reverse).
2. Compare that card's rust/ygo-cards/src/cards/c<CODE>.rs with its Lua line by line. Most bugs are a
   wrong default argument (`lua_get<T, default>` in MAIN/build/ocgcore-src/lib*.cpp; an explicit
   `nil` boolean is `false`), a wrong event code, a missing or extra call, a wrong location/player.
3. Fix it, rebuild, rerun, until the output ends with `300/300 games identical`.

Rules: edit only card files rust/ygo-cards/src/cards/c*.rs; do not commit. If the cause is in the
library (rust/ygo-cards/src/aux, procs, lua, cost.rs, chain.rs) or the core (rust/ygo-core), do NOT
change it: stop and report the evidence (game, decision, messages, the suspected function and the
C++/Lua it should match). Conventions: rust/ygo-cards/TRANSLATING.md. Never read or list
/home/tsoj/Dokumente/Projects or any directory above MAIN. Never use `rm -rf`.

Reply (short): STATUS (FIXED: 300/300 / NEEDS-LIBRARY-FIX / STUCK), the cards you changed and what
was wrong (one line each), and the final ygo-diff summary line.
