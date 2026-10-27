# ygopro-core-rust

A port of [edo9300/ygopro-core](https://github.com/edo9300/ygopro-core), the Yu-Gi-Oh! duel engine
used by EDOPro, to Rust. It includes all card scripts from
[ProjectIgnis/CardScripts](https://github.com/ProjectIgnis/CardScripts), translated from Lua to Rust.

It was written so that duel states can be copied cheaply, for game-tree search in bots.
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
- **Agreement with the original:** the port is tested differentially against the C++ core
  (`libocgcore`), built with the patch described under
  [Differences from upstream ygopro-core](#differences-from-upstream-ygopro-core). Both engines play
  the same duels (same seed, decks and responses), and every message and every query result is
  compared byte for byte. The tests include random games over
  every card, games with decks built around rarely used cards, and about 57,000 recorded games of
  search and policy bots. 13,540 of the 13,542 translated cards have been activated or summoned in
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

Messages, responses and queries use ygopro-core's binary formats, with the few changes listed below,
so most code written against `libocgcore` carries over.

## Differences from upstream ygopro-core

The port reproduces ygopro-core **with a patch** from the project it was written for, not unmodified
upstream. The patch was made for an arbiter of bot matches that must not leak hidden information and for
reconstructing duels from one player's observations. That needs games that are deterministic and
whose visible behavior never depends on hidden cards or on the order cards were created in. So the
Rust engine differs from upstream ygopro-core in these ways:

**Message formats**
- `MSG_SHUFFLE_HAND` and `MSG_SHUFFLE_EXTRA`: the top bit of each card code marks a face-up card (a
  revealed card stays visible). `MSG_SHUFFLE_HAND` also lists, for each card, its position before
  the shuffle, which only its owner should be shown.
- `MSG_SHUFFLE_SET_CARD` ends with the zone each card went to, which only its owner should be shown.
- A hand card that becomes public or private again is announced like any other position change.
- A card Set from the Deck by an effect with a requirement is shown to the other player.

**Order of iteration**
- Wherever a player can see the order in which cards or effects are handled, they are handled in
  *public order*: by location and sequence, then card id, never by memory address or creation order.
  This covers script groups, events handled one card at a time, random selections, effect option
  lists (ordered by description) and equip cards leaving together.

**Rules and timing**
- An effect gets a new timestamp when its condition becomes true again only at fixed points of the
  game (each adjustment, checked in effect id order), not whenever something happens to evaluate the
  condition. Errors in such checks are muted.
- A card's effect that negates the card itself keeps applying (upstream it would stop, so the card's
  status would flip at every refresh).
- A card entering the Deck, hand or Extra Deck forgets its position and activation history, since it
  is a new object.
- Pending shuffles inside a cost, target or operation are done when it ends.
- Properties a script assumes for cards (`Card.AssumeProperty`) last until its top-level call ends.

**Robustness**
- A step that runs too many script callbacks raises a script error instead of running practically
  forever. The limit is counted differently from the patched C++ core, which counts Lua
  instructions, so `ygo-diff` skips games where either engine hits its limit.
- The sum checks of material selections are memoized, so they no longer take exponential time.

**Extensions** (`ygo-core/src/ext.rs`, the `ext_*` methods of `Duel`)
- Hooks for reconstructing a duel from one player's observations: forcing coin and dice results,
  shuffles, random selections and set-card shuffles; treating effects as activatable or cards as
  matching filters; logging which hidden cards the game read; placing, swapping or creating cards
  and setting LP or the turn. They do nothing unless called.
- `Duel::fine_steps` (off by default): `ocg_process` also stops inside card functions where the C++
  core returns (after each step that sent messages), for callers that change the duel between
  calls. Changes made while a card function waits are made again when it runs again.

The C++ side of the patch is in `oracle/` (see Differential testing).

## Differential testing

`ygo-diff` runs the original C++ core as the oracle. Its source is the submodule
`oracle/ygopro-core` (not fetched by default); `oracle/build.sh` applies `oracle/ocgcore.patch`, adds
the extension `oracle/ocgext.{h,cpp}` (a few `OCG_Ext*` functions, for example to set the order of a
deck) and builds `target/oracle/libocgcore.so` (or set `$YGO_CORE_LIB`):

```sh
git submodule update --init --recursive oracle/ygopro-core
oracle/build.sh                                  # needs meson and ninja
```

The card data comes from `$YGO_ROOT/external/BabelCDB`, the Lua scripts from
`$YGO_ROOT/external/CardScripts` (and corrected ones from `$YGO_ROOT/lua_fixes`). The scripts in `tools/` (coverage runs, failure triage, replays of
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
