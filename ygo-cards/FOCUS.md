# Getting rarely played cards tested (instructions for a focus agent)

The Rust port of ygopro-core (with the Lua card scripts translated to Rust) is tested against the
original (libocgcore.so) by `ygo-diff`, which plays duels on both and compares every message. Random
play never activated or summoned the cards named in your task: their conditions are rarely met.
Build decks and find games in which each of your cards IS used, so that it gets tested.

You work in a git worktree. MAIN=/home/tsoj/Dokumente/Projects/yugioh_AI_2 is the main checkout.
Card scripts (Lua): MAIN/external/CardScripts/official/c<CODE>.lua; card data (names, texts, types):
the SQLite files MAIN/external/BabelCDB/*.cdb (tables `datas`, `texts`). A card can be used in a deck
if MAIN/rust/ygo-cards/src/cards/c<CODE>.rs exists or the card has no script (vanilla monster).

1. Read each card's Lua and text: what does it need (materials, a Ritual Spell, a field spell,
   cards in the GY, a battle, the opponent doing something, ...)?
2. Write two .ydk decks per card into MAIN/build/focus_decks/<CODE>_a.ydk and <CODE>_b.ydk (format:
   `#main` then one code per line (40-60 cards), `#extra` then up to 15, `!side`). Deck a: 3 copies of
   your card and what makes it usable (searchers, enablers, the right types/levels/materials). Deck b:
   an opponent that lets it happen (e.g. plain vanilla monsters if the card needs a battle or a monster
   to target; spells/traps to destroy if it needs them).
3. Build ygo-diff with only those decks' cards (from your worktree's rust/ directory, foreground):
   `YGO_CARDS_ONLY="$(python3 tools/deck_codes.py A.ydk B.ydk)" CARGO_TARGET_DIR=target/repro cargo build --release -p ygo-diff`
   (copy MAIN/rust/tools/deck_codes.py, deck_plan.py, translatable.py into your rust/tools/ if missing;
   never run cargo without YGO_CARDS_ONLY).
4. Play: `YGO_ROOT=MAIN ./target/repro/release/ygo-diff --decks A.ydk,B.ydk --games 30 --seed S --focus CODE`
   `--focus` makes the random player use your card whenever a question offers it (summon, activate,
   chain, attack, yes to its effect). The output ends with per-card coverage lines `CODE: N activations,
   M summons` and `k/30 games identical`. Change the decks / seeds until your card has activations or
   summons in identical games. (`--decks` alternates the seats per game, so both decks play first. Decks are shuffled per game;
   `--no-shuffle` keeps the .ydk order, the last card on top, which `coverage.py focus` uses for your
   decks: you may order a deck so the needed cards are drawn early.)
5. If a game mismatches (the output shows the first differing message), find the cause like
   rust/ygo-cards/FIXCOV.md describes (`-v` prints the decisions); fix it if it is a card file
   (rust/ygo-cards/src/cards/c*.rs, do not commit); if it is in the library or core, stop on that
   card and report the evidence.

The decks are replayed regularly by `tools/coverage.py focus`: per pair `<CODE>_a.ydk` (or a
variant `<CODE>_c.ydk`, ...) against `<CODE>_b.ydk`, 20 games with random seeds, `--no-shuffle`, and
`--focus` set to ALL cards of the `_a` deck. A deck is good when your card is used in most such runs:
test it that way (`--games 20` with several seeds, `--no-shuffle --focus <every code of deck a>`),
not with one lucky seed. Order the `_a` deck (the last card is drawn first) so the card and its
enablers are drawn early; make deck b passive or supply what the card needs (an attack, an
activation, a summon to respond to). A card that is only placed by another card (never activated or
summoned itself) shows 0/0 in the coverage line: then say so instead of chasing it.

Rules: only write decks (in MAIN/build/focus_decks/) and card files (in your worktree). Never read or
list /home/tsoj/Dokumente/Projects or anything above MAIN. Never use `rm -rf`. Spend at most ~20 minutes
per card; some cards may be practically impossible (say why).

Reply (short), per card: USED (deck files, seed, the coverage line) / MISMATCH (what, fixed or not) /
NOT REACHED (why); plus any card files you changed.
