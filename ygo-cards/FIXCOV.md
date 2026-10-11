# Fixing coverage mismatches of a card (instructions for a fix agent)

The Rust port of ygopro-core (with the Lua card scripts translated to Rust) is tested against the
original (libocgcore.so) by `ygo-diff`, which plays a duel on both and compares every message.
Single games between human decks found games where the two engines diverge, and the card named in
your task appears in the first differing message of them. Find what is wrong and fix it.

You work in a git worktree. MAIN=/home/tsoj/Dokumente/Projects/yugioh_AI_2 is the main checkout;
your worktree lacks `external/` and `build/` (read them from MAIN: Lua scripts in
MAIN/external/CardScripts/official/c<CODE>.lua and utility.lua/proc_*.lua there, the C++ core in
MAIN/build/ocgcore-src/). The failing games are files in MAIN/build/coverage_fails/ (named in your
task); the first line of each is the ygo-diff command, the rest its output.

Reproduce one (from your worktree's rust/ directory, in the foreground):
```
YGO_ROOT=/home/tsoj/Dokumente/Projects/yugioh_AI_2 tools/repro.sh /home/tsoj/Dokumente/Projects/yugioh_AI_2/build/coverage_fails/<file>
```
(copy MAIN/rust/tools/repro.sh, deck_codes.py, deck_plan.py and translatable.py into your worktree's
rust/tools/ first if they are missing). It builds ygo-diff with only the two decks' cards (fast) and
replays the game. It stops at the first mismatch and prints the last messages (`before`/`same`) and
the differing message (core vs rust, hex). The first byte of a message is its type (MSG_* in
rust/ygo-core/src/constants.rs); card codes are little-endian u32 (`1a ec 86 00` = 0x0086ec1a =
8842266). Add `-v` to print the decisions.

1. Decode the differing message and the messages before it: which card was activated or resolving,
   what the core did that Rust did not (or the reverse).
2. Compare that card's rust/ygo-cards/src/cards/c<CODE>.rs with its Lua line by line. Most bugs are a
   wrong default argument (`lua_get<T, default>` in MAIN/build/ocgcore-src/lib*.cpp; an explicit
   `nil` boolean is `false`), a wrong event code, a missing or extra call, a wrong location/player,
   a function registered with the wrong wrapper (a target needs `cost_target`, which passes chk).
3. Fix it and rerun each of your failing games. A game is fixed when it ends with `1/1 games
   identical`, or when its first mismatch moved to a later decision and is caused by another card
   (say which).

Rules: edit only card files rust/ygo-cards/src/cards/c*.rs; do not commit. If the cause is in the
library (rust/ygo-cards/src/aux, procs, lua, cost.rs, chain.rs) or the core (rust/ygo-core), do NOT
change it: stop and report the evidence (game, decision, messages, the suspected function and the
C++/Lua it should match). Conventions: rust/ygo-cards/TRANSLATING.md. Never read or list
/home/tsoj/Dokumente/Projects or any directory above MAIN. Never use `rm -rf`. Never run a cargo
build without YGO_CARDS_ONLY set (repro.sh sets it): the full crate needs too much memory.

Reply (short): STATUS (FIXED / PARTLY / NEEDS-LIBRARY-FIX / STUCK), the cards you changed and what
was wrong (one line each), and for each failing game its final result line.
