# ygopro-core-rust

A port of [edo9300/ygopro-core](https://github.com/edo9300/ygopro-core), the Yu-Gi-Oh! duel engine
used by EDOPro, to Rust. It includes all card scripts from
[ProjectIgnis/CardScripts](https://github.com/ProjectIgnis/CardScripts), translated from Lua to Rust.

It was written so that duel states can be copied cheaply, for game-tree search and other AI use.
A `Duel` is plain data and implements `Clone`. Cards and effects are copy-on-write, so a copy costs
a reference count per card and effect (about 5 µs for a duel in progress) instead of forking a
process or serializing a Lua state.

## Credit

This project is a translation of other people's work:

- **[ygopro-core](https://github.com/edo9300/ygopro-core)** by Edoardo Lolletti (edo9300) and the
  ygopro-core contributors (see that repository's version history). The engine in `ygo-core` is
  ported from it function by function, keeping its names and structure.
- **[CardScripts](https://github.com/ProjectIgnis/CardScripts)** by the Project Ignis contributors.
  Every card in `ygo-cards/src/cards/` is a translation of one of their Lua scripts, keeping the
  original Lua as comments next to the Rust. The shared procedures in `ygo-cards/src/aux` and
  `ygo-cards/src/procs` are ported from their `utility.lua` and `proc_*.lua` files.

Card data (names, stats, texts) is not included. It is read at runtime from the SQLite card
databases of [ProjectIgnis/BabelCDB](https://github.com/ProjectIgnis/BabelCDB).

Yu-Gi-Oh! is a trademark of Shueisha and Konami. This project is not affiliated with or endorsed by
Shueisha, Konami, edo9300 or Project Ignis.

## Status

- **Cards:** all 13,541 official card scripts are translated.
- **Agreement with the original:** the port is tested differentially against the original C++ core
  (`libocgcore`). Both engines play the same duels (same seed, decks and responses), and every
  message and every query result is compared byte for byte. The tests include random games over
  every card, games with decks built around rarely used cards, and about 57,000 recorded games of
  search and policy agents. 13,540 of the 13,542 translated cards have been activated or summoned in
  such games with both engines agreeing. Known differences that come from bugs in the C++ core are
  listed in `tools/known_core_failures.txt`.
- **Speed:** about 5× faster than the C++ core with Lua on random games, and much faster on
  script-heavy cases such as Fusion material checks.

This is an engine library, not a game client: there is no UI and no network play.

## Layout

| Crate | Contents |
| --- | --- |
| `ygo-core` | The engine: duel state, processor, field rules, card and effect logic, and an API that mirrors ygopro-core's `ocgapi` (`ocg_new_card`, `ocg_start_duel`, `ocg_process`, `ocg_get_message`, `ocg_set_response`, `ocg_query*`). |
| `ygo-cards` | The card scripts, plus the Lua utility library (`aux`) and summoning procedures (`procs`) they use. `ygo_cards::scripts()` returns the table of all cards. |
| `ygo-diff` | The differential tester. It runs ygo-core next to the C++ `libocgcore` (loaded with `libloading`) and reports the first message that differs. |

`DESIGN.md` describes how the port maps the C++ and the Lua onto Rust, including how card functions
that wait for a player's response are resumed. `ygo-cards/TRANSLATING.md` holds the conventions for
translating card scripts.

## Building

```sh
cargo build --release
```

A full release build of `ygo-cards` takes a while and about 12 GB of memory, since it compiles more
than 13,000 card files. During development, `YGO_CARDS_ONLY="c12345 c67890"` builds only the listed
cards and the cards they reference.

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
    let messages = duel.ocg_get_message().to_vec(); // same format as OCG_DuelGetMessage
    // ... read the messages; on a question, answer it:
    match status {
        Status::Awaiting => duel.ocg_set_response(&response),
        Status::End => break,
        Status::Continue => {}
    }
    let copy = duel.clone(); // cheap: search from here
}
```

Messages, responses and queries use the same binary formats as ygopro-core, so code written against
`libocgcore` carries over.

## Differential testing

`ygo-diff` needs a build of the original core and the card data. It expects
`$YGO_ROOT/build/libocgcore.so`, `$YGO_ROOT/external/BabelCDB` and the Lua scripts in
`$YGO_ROOT/external/CardScripts`. The core must be built with a small extension from the project
this port was written for. The extension adds a few `OCG_Ext*` functions, for example to set the
order of a deck, and is not part of this repository. The scripts in `tools/` (coverage runs, failure triage, replays of
recorded games) come from the larger project this port was written for, and some of them assume
its layout.

```sh
YGO_ROOT=/path/to/root ./target/release/ygo-diff --games 100 --seed 1        # random decks
YGO_ROOT=/path/to/root ./target/release/ygo-diff --decks a.ydk,b.ydk --games 20
YGO_ROOT=/path/to/root ./target/release/ygo-diff --rust-only --bench-clone   # only Rust; copy cost
```

## License

GNU Affero General Public License, version 3 or later (AGPL-3.0-or-later), the license of
ygopro-core and of the card scripts this port is derived from. See `COPYING`.
