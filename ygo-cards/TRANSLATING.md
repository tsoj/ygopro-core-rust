# Translating a card script from Lua to Rust

Each card script `external/CardScripts/official/c<code>.lua` becomes one Rust file
`rust/ygo-cards/src/cards/c<code>.rs`. The translation must behave **exactly** like the Lua
script running in ygopro-core: the same messages, the same questions, the same random numbers,
in the same order. `ygo-diff` checks this by playing random duels on both engines and comparing
everything (see "Testing").

So translate line by line. Do not simplify, reorder, merge or "fix" anything, even when it looks
redundant or wrong: an extra `IsExistingMatchingCard` call can draw nothing, but a reordered
`Hint` or a skipped `IsRelateToEffect` changes the messages. Keep the Lua comments.

Look at the pilot cards in `src/cards/` (Pot of Greed `c55144522.rs`, Mystical Space Typhoon
`c5318639.rs`, Monster Reborn `c83764718.rs`, Sangan `c26202165.rs`, Trap Hole `c4206964.rs`, ...)
before you start.

## The file

```rust
//! Pot of Greed
use super::prelude::*;

pub const ID: u32 = 55144522;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
```

- `local s,id=GetID()` becomes `pub const ID: u32 = <code>;`. Use `ID` wherever the Lua uses `id`.
- `s.initial_effect(c)` becomes `pub fn initial_effect(d: &mut Duel, c: CardId)`.
- The other `s.<name>` functions become private `fn <name>` (snake_case, the same names). The
  exceptions are names that clash with the prelude's wrappers: `s.condition` becomes `fn cond`
  and `s.operation` becomes `fn op`.
- `use super::prelude::*;` imports everything you need: the constants, the API modules `card`,
  `duel`, `effect`, `group`, `debug`, the library modules `aux`, `cost`, `chain`, and the
  helpers below.
- A new file is registered automatically (by `build.rs`). Do not edit other files.

## Declarations: `INFO`

When the Lua sets fields of the card table (`s.listed_names={...}`, `s.listed_series={...}`,
`s.counter_place_list={...}`, ...), declare them:

```rust
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLUEEYES_W_DRAGON, 12345678], listed_series: &[SET_BLUE_EYES], ..ScriptInfo::DEFAULT };
```

The fields are in `ScriptInfo` (`src/lib.rs`). If the script declares a field `ScriptInfo` does not
have, stop and report it (see "When something is missing").

## API names

The Lua API maps to Rust functions with the same names in snake_case. The first argument is
always `d: &mut Duel` (or `&Duel`):

| Lua                                     | Rust                                               |
|-----------------------------------------|----------------------------------------------------|
| `c:IsLocation(LOCATION_HAND)`           | `card::is_location(d, c, LOCATION_HAND)`           |
| `Duel.Destroy(g, REASON_EFFECT)`        | `duel::destroy(d, g, REASON_EFFECT, None, None)`   |
| `e:SetCode(EVENT_FREE_CHAIN)`           | `effect::set_code(d, e, EVENT_FREE_CHAIN)`         |
| `g:GetFirst()`                          | `group::get_first(d, &g)`                          |
| `aux.Stringid(id, 0)`                   | `aux::stringid(ID, 0)`                             |
| `Cost.SelfDiscard`                      | `cost::self_discard()`                             |
| `Duel.GetChainInfo(0, A, B)`            | `duel::get_chain_info(d, 0, &[A, B])` (a `Vec<Value>`) |
| `Duel.GetMZoneCount(tp, g, tp)`         | `duel::get_m_zone_count(d, tp, (&g).into(), Some(tp), None, None).0` |
| `Group.CreateGroup()`                   | `CardSet::new()`                                   |
| `g:Merge(g2)` / `g:Sub(g2)`             | `group::add_card(&mut g, &g2)` / `group::remove_card(&mut g, &g2)` |
| `for tc in aux.Next(g)` / `g:Iter()`    | `for tc in group::iter(d, &g)` (keep a `g:GetFirst()` before it as `let _ = group::get_first(d, &g);`) |
| `Duel.GetOperationInfo(ev, cat)`        | `duel::get_operation_info(d, ev, cat)`: `None` = `ex` false, else `OpTarget { op_cards, op_count, op_player, op_param }` |
| `Duel.NegateEffect(ev)`                 | `duel::negate_effect(d, a.ev as u8)`               |
| `Duel.SelectEffect(tp, {b1,s1}, {b2,s2})` | `duel::select_effect(d, tp, &[(b1, s1), (b2, s2)])` (`Option<usize>`, 1-based like Lua) |
| `Duel.AddCustomActivityCounter(id, ACTIVITY_CHAIN, function(re,tp,cid) ... end)` | `duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(f))`, `f(d, a: Args)` reads `re` as `a.e(0)` |

Find the exact signature with `grep -rn "pub fn <name>(" rust/ygo-cards/src rust/ygo-core/src/api`
(from the repository root). The module of a function (`card::`, `duel::`, ...) is the
`ygo-cards` facade in `src/lua/`. It re-exports the core API (`rust/ygo-core/src/api/`) and replaces
the functions that `utility.lua`, `proc_workaround.lua`, ... redefine. Always call through these
modules, never `ygo_core::api::...` directly: e.g. `card::register_effect` and
`duel::register_effect` are the Lua-wrapped versions.

- **Optional arguments** are `Option`s: pass `None` where the Lua leaves the argument out or passes
  `nil`. Look at the C++ (`build/ocgcore-src/lib*.cpp`), the Lua libraries
  (`external/CardScripts/utility.lua`, `proc_*.lua`: `Duel.SelectEffect`, `aux.*` and more are
  defined there), or the Rust doc comment when unsure what a default means. Where the Rust
  function takes a plain value for a Lua argument with a default, pass that default: a
  one-argument `Duel.HintSelection(g)` is `duel::hint_selection(d, g, true)`.
- A Lua **number where the C++ also accepts a table** (set codes in
  `Duel.IsPlayerCanSpecialSummonMonster(tp, code, 0, ...)`) is a list of one value:
  `setcodes: Some(vec![0])`. Only `nil` is "no value".
- **Lua `nil` checks:** `if tc and tc:IsX() then` is `if let Some(tc) = tc { if ... }`;
  `(x or 0)` on an `Option` is `x.map_or(0, ...)`.
- **Summon context variants:** functions that take the extra `scard, sumtype, playerid` arguments
  (`c:IsAttribute(att, scard, sumtype, tp)`, ...) have a `_ctx` version:
  `card::is_attribute_ctx(d, c, att, scard, sumtype, Some(tp))`.
- **Several return values** come back as a tuple or a `Vec`. Lua `local a,b=f()` usually reads the
  first ones. `duel::get_first_target(d)` returns all targets; `duel::first_target(d)` returns the
  first one as an `Option`. A second value the function may not return
  (`local code1,code2=c:GetOriginalCodeRule()`) is `nil` in Lua: use `v.get(1).copied()` and compare
  with `Some(x)`, so `nil == x` stays false.
- **Integers:** constants have the types the core uses (`u32` for most, `u64` for
  `CATEGORY_*`/`RACE_*`/hint descriptions). Cast with `as` where a signature needs another width
  (`HINT_SELECTMSG as u8`, `HINTMSG_DESTROY as u64`, `POS_FACEUP as u8`). The compiler tells you.
- **Lua's `1-tp`** is `1 - a.tp` (players are `u8`).
- The `param` of `duel::set_operation_info` is an `i32`: `LOCATION_DECK as i32`. A numeric
  effect value is an `i64`: `effect::set_value(d, e, RESET_TURN_SET as i64)`.
- `e:SetReset(v)` without a count is `effect::set_reset(d, e, v, 1)`.
- **Borrowing:** `d` cannot be borrowed twice in one call. Read values into locals first:
  `let label = effect::get_label(d, a.e); duel::raise_event(d, ..., label as u32);`.
- A Lua call that returns several values, used where Lua takes only the first (`c:GetCode()` in
  `c:IsCode(tc:GetCode())` keeps all of them, `local code=tc:GetCode()` only the first): take
  `[0]`/`.first()` of the Rust `Vec` where the Lua takes one value.
- `Duel.AnnounceCard(tp, ...)` takes the filter as an opcode list:
  `duel::announce_card(d, tp, &[code1, OPCODE_ISCODE, code2, OPCODE_ISCODE, OPCODE_OR])`.
- **Lua table order:** `pairs`/`next` over a table with number keys (e.g. `ids[tc:GetCode()]=true`
  then `for code in pairs(ids)`) does not go in insertion or sorted order. Build the table with
  `luatable::LuaTable` (`t.set(k, Some(v))`, `t.pairs()`), which has Lua 5.4's exact order.
  `ipairs` over a list (`{a, b, c}`, `table.insert`) is just the list order.
- **Keep every call**, even when the Lua stores the result in a variable it never uses
  (`local ph=Duel.GetCurrentPhase()` becomes `let _ph = duel::get_current_phase(d);`).

## Effect functions

Conditions, costs, targets and operations are Rust functions wrapped into a `LuaFn`:

| Lua                                   | Rust function                                 | wrapper                |
|---------------------------------------|-----------------------------------------------|------------------------|
| `function(e,tp,eg,ep,ev,re,r,rp)` condition | `fn(d: &mut Duel, a: &EffArgs) -> bool` | `condition(f)`         |
| `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)` cost/target | `fn(d: &mut Duel, a: &EffArgs) -> bool` | `cost_target(f)` |
| `function(e,tp,eg,ep,ev,re,r,rp)` operation | `fn(d: &mut Duel, a: &EffArgs)`         | `operation(f)`         |
| `function(e,c)` (target of a field effect, ...) | `fn(d, e: EffectId, c: CardId) -> bool` | `effect_card_fn(f)` |
| `function(e,c)` returning a number    | `fn(d, e, c) -> i64`                          | `effect_card_value(f)` |
| anything else (`function(e,re,tp)`, ...) | `fn(d: &mut Duel, a: Args) -> R` (`R`: `bool`, an integer, `()`, ...) | `lua_fn(f)` |

`cost_target` also wraps costs and targets that take only `chk` (no `chkc`).

`EffArgs` has the fields `e, tp, eg, ep, ev, re, r, rp, chk`:
- `a.eg` is an `Option<CardSet>`: `None` if the Lua `eg` is `nil`. Where the Lua indexes `eg`
  without checking it (`eg:IsExists(...)`, `eg:GetFirst()`), use `a.eg.clone().nil()`, which
  errors like Lua on `nil`. Where the Lua only passes `eg` on (`Duel.RaiseEvent(eg, ...)`), use
  `a.eg()`, which gives the group or an empty one.
- `a.chk` is `Some(0)` in the check call of a cost/target, `Some(1)` otherwise.
- `a.chkc()` is the Lua `chkc` (an `Option<CardId>`).
- `a.re` is an `Option<EffectId>`: `a.re.nil()` where the Lua indexes `re`.
- Where the Lua passes `eg` on to a parameter that accepts `nil` (`Duel.SetOperationInfo(0, cat,
  eg, ...)`), pass `a.eg.clone()`.
- Helper functions that return a Lua `function(...)` return `impl Fn(...)` in Rust.

`Args` reads arguments by position: `a.e(0)` (an effect), `a.c(1)` (a card), `a.card(1)` (a card
or nil), `a.p(2)` (a player), `a.int(i)`, `a.g(i)`. Unlike Lua, numbering starts at 0.

Translate the Lua checks exactly as written:
```lua
if chkc then return chkc:IsOnField() and chkc~=c end
if chk==0 then return Duel.IsExistingTarget(...) end
```
```rust
if let Some(chkc) = a.chkc() {
    return card::is_on_field(d, chkc) && chkc != c;
}
if a.chk == Some(0) {
    return duel::is_existing_target(d, ...);
}
```
A target/cost function that falls off the end in Lua returns `true` in Rust (the value is not
used).

A **value** that is a number is `effect::set_value(d, e, 1000)`. A value that is a function is
`effect::set_value_fn(d, e, lua_fn(f))` (or one of the other wrappers above).

Reuse the library's ready-made functions where the Lua does: `aux.bdocon` becomes
`aux::bdocon()`, `aux.chainreg` becomes `aux::chainreg()`, `Cost.SelfBanish` becomes
`cost::self_banish()`, and so on. Look in `src/aux/`, `src/cost.rs`, `src/chain.rs`,
`src/procs/` (see "Library").


## Library

The Lua libraries are ported as modules with the same names in snake_case. A Lua name usually
tells you where its Rust function is:

| Lua                                         | Rust                                           |
|---------------------------------------------|------------------------------------------------|
| `aux.X`, `Auxiliary.X` (utility.lua, cards_specific_functions.lua, proc_equip/normal/union/persistent.lua) | `aux::x` (`src/aux/`) |
| `Cost.X`                                    | `cost::x` (`src/cost.rs`)                      |
| `Chain.X`, `e:GetCardX()`, `e:IsCardX(...)`  | `chain::...` (`src/chain.rs`; generic by `Prop`) |
| `Xyz.X`, `Link.X`, `Synchro.X`, `Fusion.X`, `Pendulum.X`, `Ritual.X`, `Gemini.X`, `Spirit.X` | `xyz::x`, `link::x`, ... (`src/procs/`) |
| `Witchcrafter.X`, `Cyberdark.X`, `Drytron.X`, `AA.X`, ... | `aux::witchcrafter::x`, `aux::cyberdark::x`, ... |
| deprecated aliases (deprecated_functions.lua: `aux.SelfToGraveCost`, ...) | the new function (`cost::self_to_grave()`) |

Procedure examples:

```rust
// Xyz.AddProcedure(c,nil,4,2)
xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
// Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
// Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
// Fusion.AddProcMix(c,true,true,CARD_A,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_A), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp))))]);
// Fusion.AddProcMixN(c,true,true,X,2)
fusion::add_proc_mix_n(d, c, true, true, vec![(X_mat, 2)]);
// Fusion.RegisterSummonEff(c) / Fusion.CreateSummonEff({handler=c,fusfilter=...,extraop=Fusion.BanishMaterial})
fusion::register_summon_eff(d, c, fusion::SummonEff::default());
fusion::create_summon_eff(d, c, fusion::SummonEff { fusfilter: Some(fusion::fus_filter(|d, c, tp| ...)), extraop: Some(fusion::banish_material()), ..Default::default() });
// Pendulum.AddProcedure(c)
pendulum::add_procedure(d, c, None, None);
// Ritual.AddProcGreaterCode(c,7,nil,CODE) / Ritual.AddProcGreater({handler=c,filter=...})
ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[CODE]);
// aux.AddEquipProcedure(c,0,aux.FilterBoolFunction(Card.IsRace,RACE_WARRIOR))
aux::add_equip_procedure(d, c, Some(0), Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_WARRIOR))), None, None, None, None, None, None);
```

Lua functions with named arguments (`Fusion.CreateSummonEff({handler=c, ...})`, `Ritual.AddProc`,
`Fusion.AddContactProc`) take a parameter struct in Rust (`fusion::SummonEff`,
`ritual::RitualProc`, `fusion::Contact`): set the fields the Lua sets, `..Default::default()`
for the rest. Positional Lua calls of the same functions fill the same fields in order.

Some library callbacks may **change the group they get** (Lua groups are references): an
`extraop` of a Fusion Summon (`function(e,tc,tp,sg) ... sg:Clear() end`) is written with
`fusion::extra_op(|d, e, tc, tp, sg| { ...; true })`, where `sg: &mut CardSet`; return `false`
only where the Lua returns `false`.

The procedures set fields of the card table themselves (`s.material`, `minxyzct`,
`min_tribute_req`, `fit_monster`, ...): do not declare those in `INFO` unless the Lua script sets
them itself. Fields that are functions (`s.mat_filter`, `s.ritual_custom_check`) go into `INFO`
as function pointers: `mat_filter: Some(mat_filter_fn)`.

Other cards' tables: `Duel.GetMetatable(code).x` / `c.x` read the declarations of another
script: `crate::script_info(code)` / `crate::card_info(d, c)`.

## Filters

A Lua filter `function s.filter(c, e, tp)` becomes a Rust function
`fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool`. API functions take filters as
`Option<Filter>` (`Filter = &dyn Fn(&mut Duel, CardId) -> bool`). Pass the extra arguments with a
closure:

```rust
duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
```

- A filter without extra arguments can be passed directly: `Some(&filter)`, `Some(&card::is_spell_trap)`.
- `nil` or `aux.TRUE` as the filter is `None`.
- The `ex` argument (a card or group to leave out) is `NO_EX` for `nil`, `c.into()` for a card,
  `(&g).into()` for a group.
- Combinators (`aux.FilterBoolFunction(Card.IsRace, RACE_DRAGON)`, `aux.FaceupFilter(...)`,
  `aux.NOT(...)`, `aux.AND(...)`) become closures: `&|d, c| card::is_race(d, c, RACE_DRAGON)`.
  The same applies to `aux.TargetBoolFunction` (a `function(e, c)`): `effect_card_fn(|d, _e, c| ...)`.

## Groups

Groups are `CardSet` values. Lua groups are references, Rust groups are values:
- `#g` is `g.len()`. `#g>0` is `!g.is_empty()`.
- Iterate with `for tc in group::iter(d, &g)`, which uses the order of `GetFirst`/`GetNext`/`Iter`.
  `g:GetFirst()` is `group::get_first(d, &g)`.
- Functions that change a group take `&mut CardSet` (`group::add_card`, `group::remove_card`,
  `group::match_`, `group::remove`).
- **After you change a group that is also stored somewhere else** (a label object, a script
  variable), store it again. The stored copy does not change with yours.
- `g:Select(...)`, `Duel.SelectMatchingCard(...)`, ... return `Option<CardSet>`: `None` if the
  player canceled (only possible when `cancelable` is true). Lua's `nil`. When the selection is
  not cancelable, write `.nil()` where the Lua indexes the result (`#g` on `nil` would be an error
  in Lua too). Where the Lua only passes it on (`local g=Duel.SelectTarget(...)` then
  `Duel.SetOperationInfo(0, cat, g, ...)`), pass the `Option` as it is.
- `Duel.SelectMatchingCard` with 8 arguments and `g:Select` with 5 are not cancelable: pass `false`.

## Errors

When the Lua would raise an error (e.g. indexing `nil`: `local tc=Duel.GetFirstTarget()` with no
target, then `tc:IsRelateToEffect(e)`), raise the same error: `option.nil()` returns the value, or
raises a Lua error, which the engine handles like ygopro-core does. Use `.nil()` for every value
that the Lua indexes without checking it. `effect::get_handler(d, a.e).nil()` is the usual
`e:GetHandler()`. Never use `.unwrap()` on something Lua could see as `nil`.

## State

**All state must live in the duel**, never in Rust `static`s, `thread_local`s, or variables
captured by closures and changed later. The engine copies duels (that is the point of this port),
and state kept elsewhere would not be copied.

- Lua `s.x` / `s[0]` (variables on the card table) become `s_var(d, ID, key)` and
  `set_s_var(d, ID, key, Value::Int(v))`, where `key` is a number you choose per variable.
- `aux.GlobalCheck(s, function() ... end)` becomes `aux::global_check(d, ID, |d| { ... })`.
- Effect labels (`e:SetLabel`, `e:SetLabelObject`) are already state in the duel. Use them as the
  Lua does.
- Closures may capture **constant** values: codes, the effect or card ids from `initial_effect`,
  numbers. That is how Lua upvalues that are never reassigned translate.

## When something is missing

The library (`src/aux/`, `src/cost.rs`, `src/lua/`, `src/procs/`) is a port of the Lua
libraries (`utility.lua`, `proc_*.lua`, ...), and it may still miss a few functions. If the card needs a
library function, a `ScriptInfo` field or an API function that does not exist, **do not add it
to the library or the core yourself**. Leave the card unfinished, and report exactly what is
missing (the Lua name and where it is defined). Do not work around a missing function by
inlining its Lua code into the card, unless the card's own script defines it.

## Testing

Build, then play random duels with decks that mix vanilla monsters and the card:

```sh
cd rust
cargo build --release
./target/release/ygo-diff --cards 55144522 --scripted 0.3 --games 300 --seed 1
```

- `--cards` is a comma-separated list of cards for the decks: your card, plus any already
  translated cards it works with (`ls rust/ygo-cards/src/cards`) and any Normal Monsters without a
  script. Extra Deck monsters go into an Extra Deck (`--extra N` cards, 15 by default).
- `--scripted` is the share of those cards in the decks.
- The output must end with `N/N games identical`.
- The coverage lines show how often each card was activated and summoned. An effect that never
  ran is not tested. Then raise `--games`, `--scripted`, or add partner cards. If the card needs a
  partner that is not translated yet (e.g. a Field Spell for a card that searches one), say so in
  your report: that is not a failure of your translation.
- A mismatch prints the game, the decision, the differing message, and the messages before it.
  The first byte of a message is its type (`MSG_*` in `rust/ygo-core/src/constants.rs`). Rerun
  one game with `--seed <seed> --games 1 -v`. Compare your translation with the Lua line by line
  around the place that produces that message. Most mismatches are a missing or extra call, a
  wrong default argument, or a wrong `ex`/location argument.
- The game seed of a mismatch shows in the output: `--seed S --games G` replays the same games.
- The coverage lines count activations and summons per card, not branches inside an operation. To
  see whether a branch ran, add a temporary `eprintln!` to your card and remove it afterwards.

Finish when the card's effects run in the coverage stats and several hundred games are identical
(more for cards whose effects rarely run).
