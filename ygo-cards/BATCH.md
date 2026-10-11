# Batch translation (instructions for a translation agent)

You translate a batch of Yu-Gi-Oh! card scripts from Lua to Rust. Work quickly. **Do not run any duel
tests.** Only make the cards compile; the main session tests the batch afterwards with the decks that
use these cards.

You work in a git worktree of the repository. MAIN=/home/tsoj/Dokumente/Projects/yugioh_AI_2 is the
main checkout. Your worktree lacks `external/` and `build/`: read the Lua from
MAIN/external/CardScripts/official/c<CODE>.lua (if it is not there: pre-release/, pre-errata/ or goat/, the
order the core uses; library: MAIN/external/CardScripts/utility.lua,
proc_*.lua, cards_specific_functions.lua) and the C++ core from MAIN/build/ocgcore-src/.

Steps:
1. Read rust/ygo-cards/TRANSLATING.md once, completely. Hundreds of cards are already translated in
   rust/ygo-cards/src/cards/: look at two or three for the style, and grep them whenever a Lua
   pattern puzzles you.
2. For each card write rust/ygo-cards/src/cards/c<CODE>.rs, a faithful line-by-line translation
   (every call, the same order, the same event codes). Find signatures with grep in
   rust/ygo-cards/src and rust/ygo-core/src/api.
3. **Default arguments are the main source of bugs.** Whenever the Lua leaves out an argument, look
   the default up (C++: `lua_get<T, default>` in MAIN/build/ocgcore-src/lib*.cpp; or the Lua
   library). Never guess. An explicit `nil` passed to a boolean parameter is `false`, not the default.
4. Compile after every two or three cards:
   `cd rust && YGO_CARDS_ONLY="<your codes, space-separated>" CARGO_BUILD_JOBS=2 cargo check -p ygo-cards 2>&1 | grep -E "^(warning|error)" -A7 | head -80`.
   YGO_CARDS_ONLY makes the build compile only those card files (the full crate needs ~8 GB of
   memory, and many agents run at once): always set it, listing all your codes.
   Fix all errors and warnings in your files.
5. If a card needs a library function, ScriptInfo field or API function that does not exist, do not
   add it and do not work around it: delete that card's file, note what is missing, and continue.

Rules: create only files rust/ygo-cards/src/cards/c<CODE>.rs; edit no other file; do not commit.
Never read or list /home/tsoj/Dokumente/Projects or any directory above MAIN. Never use `rm -rf`.
Do not run ygo-diff or `cargo build --release`.

Reply (short):
STATUS: DONE (all compile) or PARTIAL
DONE: codes written
SKIPPED: code, what is missing
UNSURE: code, which Lua lines you were not sure how to translate
GUIDE: anything in TRANSLATING.md that was wrong or missing (at most 3 lines)
