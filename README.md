# ygopro-core-rust

A port of [edo9300/ygopro-core](https://github.com/edo9300/ygopro-core), the Yu-Gi-Oh! duel engine
of EDOPro, to Rust. It includes all card scripts of
[ProjectIgnis/CardScripts](https://github.com/ProjectIgnis/CardScripts), translated from Lua to Rust.
It also includes an arbiter that runs matches between Yu-Gi-Oh! bots.

The port makes copies of a duel cheap, for game-tree search in bots. A `Duel` is plain data and
implements `Clone`. Cards and effects are copy-on-write, so a copy of a duel in progress takes about
5 µs.

## Credit

This project is a translation of other people's work:

- **[ygopro-core](https://github.com/edo9300/ygopro-core)** by Edoardo Lolletti (edo9300) and the
  ygopro-core contributors (see that repository's version history). The engine in `ygo-core` is
  ported from it function by function, with the same names and structure.
- **[CardScripts](https://github.com/ProjectIgnis/CardScripts)** by the Project Ignis contributors.
  Each card in `ygo-cards/src/cards/` is a translation of one of their Lua scripts, with the
  original Lua as comments next to the Rust. The shared procedures in `ygo-cards/src/aux` and
  `ygo-cards/src/procs` are ported from their `utility.lua` and `proc_*.lua` files.

Card data (names, stats, texts) is not included. The engine reads it at runtime from the SQLite card
databases of [ProjectIgnis/BabelCDB](https://github.com/ProjectIgnis/BabelCDB).

Yu-Gi-Oh! is a trademark of Shueisha and Konami. This project is not affiliated with or endorsed by
Shueisha, Konami, edo9300 or Project Ignis.

## Status

- **Cards:** all 13,541 official card scripts are translated.
- **Agreement with the C++ core:** a differential test plays the same duels on both engines (same
  seed, decks and responses) and compares every message and query result byte for byte. The C++
  core has the patch described in
  [Differences from upstream ygopro-core](#differences-from-upstream-ygopro-core). The tests include
  random games over every card, games with decks built around rarely used cards, and about 57,000
  recorded games of bots. 13,540 of the translated cards have been activated or summoned in games
  where both engines agree. `tools/known_core_failures.txt` lists the known differences that come
  from bugs in the C++ core.
- **Speed:** about 5 times faster than the C++ core with Lua on random games, and much faster on
  script-heavy cases such as Fusion material checks.

There is no user interface and no network play.

## Layout

| Crate or folder | Contents |
| --- | --- |
| `ygo-core` | The engine: duel state, processor, rules, card and effect logic. Its API mirrors ygopro-core's `ocgapi` (`ocg_new_card`, `ocg_start_duel`, `ocg_process`, `ocg_get_message`, `ocg_set_response`, `ocg_query*`). |
| `ygo-cards` | The card scripts, and the Lua utility library (`aux`) and summoning procedures (`procs`) they use. `ygo_cards::scripts()` returns the table of all cards. |
| `ygo-arbiter` | Matches between bots: the arbiter `ygo-arbiter`, the bot protocol (`ygo-arbiter/PROTOCOL.md`) and the reference bot `ygo-random-bot`. |
| `ygo-diff` | The differential test. It runs `ygo-core` next to the C++ `libocgcore` and reports the first message that differs. |
| `oracle/` | The C++ core for the differential test, and its patch. |
| `tools/` | Scripts for the differential test (see [Differential testing](#differential-testing)). |

`DESIGN.md` describes how the port maps the C++ and the Lua to Rust, for example how a card function
that waits for a player's response continues. `ygo-cards/TRANSLATING.md` has the conventions for
translating card scripts.

## Building

```sh
cargo build --release
```

A release build of `ygo-cards` compiles more than 13,000 card files. It takes some minutes and about
12 GB of memory. During development, `YGO_CARDS_ONLY="c12345 c67890"` builds only the listed cards and
the cards they refer to.

## Using the engine

```rust
use std::{path::Path, sync::Arc};
use ygo_core::api::NewCardInfo;
use ygo_core::carddb::CardDb;
use ygo_core::constants::*;
use ygo_core::duel::{Duel, DuelOptions, PlayerOptions};
use ygo_core::processor1::Status;

let db = Arc::new(CardDb::load(Path::new("path/to/BabelCDB"))?);
let scripts = Arc::new(ygo_cards::scripts());
let player = PlayerOptions { starting_lp: 8000, starting_draw_count: 5, draw_count_per_turn: 1 };
let options = DuelOptions { seed: [1, 2, 3, 4], flags: DUEL_MODE_MR5, team1: player, team2: player };
let mut duel = Duel::new(&options, db, scripts);
ygo_cards::load_global_scripts(&mut duel);

for code in deck {
    duel.ocg_new_card(&NewCardInfo { team: 0, duelist: 0, code, con: 0, loc: LOCATION_DECK, seq: 0, pos: POS_FACEDOWN_DEFENSE });
}
// ... the other player's cards ...
duel.ocg_start_duel();
loop {
    let status = duel.ocg_process();
    let messages = duel.ocg_get_message().to_vec(); // the format of OCG_DuelGetMessage
    // ... read the messages; for a question, set the response:
    match status {
        Status::Awaiting => duel.ocg_set_response(&response),
        Status::End => break,
        Status::Continue => {}
    }
    let copy = duel.clone(); // cheap: search from here
}
```

Messages, responses and queries have ygopro-core's binary formats, with the few changes listed under
[Differences from upstream ygopro-core](#differences-from-upstream-ygopro-core). Most code written
for `libocgcore` works.

## Running bot matches

`ygo-arbiter` plays matches between two bots, like fastchess for chess engines. A bot is a program
that speaks the bot protocol: JSON lines on its standard input and output
(`ygo-arbiter/PROTOCOL.md`). The arbiter gives each bot only what its player may see.

```sh
cargo build --release -p ygo-arbiter
YGO_ROOT=/path/to/data ./target/release/ygo-arbiter \
    --bot "new=/path/to/my-bot --my-flag" \
    --bot "base=./target/release/ygo-random-bot" \
    --book book.jsonl -n 100 --tc 10+0.1 -c 8 --records games.jsonl
```

The arbiter reads the card databases from `$YGO_ROOT/external/BabelCDB`. Without `YGO_ROOT`, it uses
the folder that contains this repository.

A match goes like this:

1. The arbiter starts each bot once. It prints the bot's identity and checks the options you set.
2. It takes openings from the book. It plays each opening twice: the bots change seats, and each
   seat keeps its deck. So each bot plays each deck once, with the same draws.
3. It starts a new bot process for each game.
4. After each pair of games, it prints the score of the first bot:
   `games 40: +20 =0 -20  elo -0.0 +/- 98.9  penta [4, 0, 12, 0, 4]`. `penta` counts the pairs by
   their score, from 0 to 2 in steps of 0.5. With `--sprt`, the line also shows the LLR.

| Option | Meaning |
|---|---|
| `--bot NAME=COMMAND` | A bot: its name, and the command that starts it. Give exactly two. The first is the one the score is for. |
| `--option NAME.OPTION=VALUE` | Set an option that the bot declares (`PROTOCOL.md`), for example `--option new.Hash=256`. |
| `--book FILE` | The opening book. Required. |
| `-n N` | The number of openings (default 100). The match has 2N games. |
| `--shuffle SEED` | Shuffle the openings with this seed first. Without it, the arbiter takes the first openings of the book. |
| `--pool N` | Use only the first N lines of the book. |
| `--book-min N` | Use only openings in which each seat won at least N test games (`wins`; default 2). |
| `--tc BASE+INC` | Time control: BASE seconds, plus INC seconds after each decision. Without it, there is no clock. |
| `--max-turns N` | A game is a draw after N turns (default 100). |
| `--sprt ELO0 ELO1` | Stop when the SPRT accepts H0 (Elo ELO0) or H1 (Elo ELO1). `--alpha` and `--beta` set the error rates (default 0.05). |
| `-c N` | The number of games at the same time (default: the number of CPU threads). |
| `--records FILE` | Write each game to FILE, as one JSON line: the decks, the seeds, all responses and the result. |
| `--resume FILE` | Continue a stopped match. FILE is its `--records` file; its finished pairs count. |
| `--nps-nodes N` | A node limit for each decision: the second bot searches N nodes, the first N times the ratio of their speeds (nodes per second). |

An opening book has one JSON object per line:

```json
{"seed": 4207, "decks": [{"name": "Deck A", "main": [89631139, ...], "extra": [...]}, {"name": "Deck B", "main": [...], "extra": [...]}], "wins": [6, 4]}
```

| Member | Meaning |
|---|---|
| `seed` | Sets both deck orders and the random numbers of the duel. |
| `decks` | The decks of player 0 and player 1, as card codes. Player 0 starts. |
| `wins` | Optional. The wins of each seat in test games, for `--book-min`. |

## Differences from upstream ygopro-core

This port has a **patch** that upstream ygopro-core does not have. The patch is for bot matches
that must not leak hidden information, and for the reconstruction of a duel from one player's
observations. These need games that are deterministic, and whose visible course never depends on
hidden cards or on the order in which cards were created.

**Message formats**
- `MSG_SHUFFLE_HAND` and `MSG_SHUFFLE_EXTRA`: the top bit of each card code marks a face-up card (a
  revealed card stays visible). `MSG_SHUFFLE_HAND` also gives each card's position before the
  shuffle; only its owner may see it.
- `MSG_SHUFFLE_SET_CARD` ends with the zone that each card went to; only its owner may see it.
- A hand card that becomes public or private again is announced as a position change.
- A card Set from the Deck by an effect with a requirement is shown to the other player.

**Order**
- Where a player can see the order in which cards or effects are handled, the engine uses the
  *public order*: by location and sequence, then by card id. It never uses memory addresses or the
  creation order. This applies to script groups, events for one card at a time, random selections,
  effect option lists (by description) and equip cards that leave together.

**Rules and timing**
- An effect gets a new timestamp when its condition becomes true again only at fixed points (each
  adjustment, in effect id order), not each time something reads the condition. Errors in these
  checks are muted.
- A card's effect that negates the card itself continues to apply. Upstream, it stops, and the card's
  status changes at each refresh.
- A card that enters the Deck, hand or Extra Deck forgets its position and activation history,
  because it is a new object.
- Pending shuffles inside a cost, target or operation are done when it ends.
- Properties that a script assumes for cards (`Card.AssumeProperty`) last until its top-level call ends.

**Robustness**
- A step that runs too many script callbacks raises a script error instead of running almost
  forever. The patched C++ core counts Lua instructions instead, so `ygo-diff` skips games in which
  either engine reaches its limit.
- The sum checks of material selections are memoized, so they no longer take exponential time.

**Extensions** (`ygo-core/src/ext.rs`, the `ext_*` methods of `Duel`)
- Hooks for the reconstruction of a duel from one player's observations: force coin and dice
  results, shuffles, random selections and set-card shuffles; treat effects as activatable or cards
  as matching filters; log which hidden cards the game read; place, swap or create cards; set LP or
  the turn. They do nothing unless called.
- `Duel::fine_steps` (off by default): `ocg_process` also stops inside card functions where the C++
  core returns (after each step that sent messages). This is for callers that change the duel
  between calls. Changes made while a card function waits are made again when it runs again.

`oracle/` has the C++ side of the patch.

## Differential testing

`ygo-diff` uses the original C++ core as the oracle. Its source is the submodule
`oracle/ygopro-core` (not fetched by default). `oracle/build.sh` applies `oracle/ocgcore.patch`, adds
the extension `oracle/ocgext.{h,cpp}` (a few `OCG_Ext*` functions, for example to set the order of a
deck), and builds `target/oracle/libocgcore.so` (or set `$YGO_CORE_LIB`):

```sh
git submodule update --init --recursive oracle/ygopro-core
oracle/build.sh                                  # needs meson and ninja
```

`ygo-diff` reads the card data from `$YGO_ROOT/external/BabelCDB` and the Lua scripts from
`$YGO_ROOT/external/CardScripts`, with corrected scripts from `$YGO_ROOT/lua_fixes`:

```sh
YGO_ROOT=/path/to/data ./target/release/ygo-diff --games 100 --seed 1        # random decks
YGO_ROOT=/path/to/data ./target/release/ygo-diff --decks a.ydk,b.ydk --games 20
YGO_ROOT=/path/to/data ./target/release/ygo-diff --rust-only --bench-clone   # Rust only: the cost of a copy
```

The scripts in `tools/` run larger tests: `coverage.py` (random games between book decks, until each
card was used), `triage.py` (which cards cause the mismatches), `repro.sh` (replay one mismatch),
`replay_records.py` (replay the recorded games of a bot match on both engines). They expect this
repository as a submodule of a project that has `external/`, `lua_fixes/`, `runs/books/` and
`build/`.

## License

GNU Affero General Public License, version 3 or later (AGPL-3.0-or-later), the license of
ygopro-core and of the card scripts this port is derived from. See `COPYING`.
