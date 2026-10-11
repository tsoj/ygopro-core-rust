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
  `s.operation` becomes `fn op`, `s.cost` becomes `fn cost_fn` (`cost` is the library module), and `s.drop` becomes `fn drop_op` (`drop` is `std::mem::drop`).
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
`s.roll_dice=true` / `s.toss_coin=true` are `roll_dice: true` / `toss_coin: true`; `s.LVnum=4`
`s.LVset=SET_X` is `lv: Some((4, SET_X))`. `s.xyz_number=39` is `xyz_number: Some(39)` (likewise the other number fields).

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
| `Chain.IsTriggeringSetcode(ch, s)` / `Chain.IsTriggeringType(ch, t)` / `Chain.IsTriggering<Prop>(...)` | `chain::is_triggering(d, ch, chain::Prop::SetCard, &[s as i64])` (`Prop::Type`, ...; a `Value`: `Nil` without the chain link, else `Bool`) |
| `Chain.GetCurrentLink()`                | `duel::get_current_chain(d, false)` |
| a library-table function the script defines itself (`function Cost.SelfTribute2(...)`) | a private fn in the card file; a script-set field such as `aux.RitualSummoningLevel=lv` is `set_lua_global(d, "RitualSummoningLevel", Value::Int(lv))` |
| `Duel.GetChainInfo(0, A, B)`            | `duel::get_chain_info(d, 0, &[A, B])` (a `Vec<Value>`) |
| `Duel.GetMZoneCount(tp, g, tp)`         | `duel::get_m_zone_count(d, tp, (&g).into(), Some(tp), None, None).0` |
| `Group.CreateGroup()`                   | `CardSet::new()`                                   |
| `g:Merge(g2)` / `g:Sub(g2)`             | `group::add_card(&mut g, &g2)` / `group::remove_card(&mut g, &g2)` |
| `for tc in aux.Next(g)` / `g:Iter()`    | `for tc in group::iter(d, &g)` (if the Lua has a `local tc=g:GetFirst()` before it, keep it as `let _ = group::get_first(d, &g);`) |
| `Duel.GetOperationInfo(ev, cat)`        | `duel::get_operation_info(d, ev, cat)`: `None` = `ex` false, else `OpTarget { op_cards, op_count, op_player, op_param }` |
| `Duel.AnnounceLevel(p, min, max, ...)`  | `duel::announce_number_range(d, p, Some(min), Some(max), &[...])` (the C++ alias) |
| `Duel.NegateEffect(ev)`                 | `duel::negate_effect(d, a.ev as u8)`               |
| `Duel.SelectEffect(tp, {b1,s1}, {b2,s2})` | `duel::select_effect(d, tp, &[(b1, s1), (b2, s2)])` (`Option<usize>`, 1-based like Lua) |
| `Duel.AddCustomActivityCounter(id, ACTIVITY_CHAIN, function(re,tp,cid) ... end)` | `duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(f))`, `f(d, a: Args)` reads `re` as `a.e(0)` |
| `c:IsSetCard(SET_X)`                    | `card::is_set_card(d, c, &[SET_X as u16])` (set codes are `u16` here); with the Lua extra args `(SET_X, lc, sumtype, tp)`: `card::is_set_card_ctx` |
| `Duel.ChangePosition(c, au, 0, du, 0)`  | `duel::change_position(d, c, au, Some(0), Some(du), Some(0), false)`: an omitted `ad`/`du`/`dd` is `None` (= `au`), an explicit `0` is `Some(0)` |
| `Cost.DetachFromSelf(n)` / `(n, m)`      | `cost::detach_from_self(cost::Count::N(n), None, None)` / `(Count::N(n), Some(Count::N(m)), None)` |
| `Duel.IsMainPhase()` / `Duel.IsBattlePhase()` | `duel::is_main_phase(d, None)` / `duel::is_battle_phase(d, None)` |
| `Duel.SSet(tp, g)`                      | `duel::s_set(d, tp, g, None, true)` (toplayer = tp, confirm = true) |
| `c:GetMaterial()`                       | `card::get_material(d, c)` (the material group; `c.material` is something else: see below) |
| `e:SetLabelObject(tc)` / `e:GetLabelObject()` (a card) | `effect::set_label_object(d, e, Value::Card(tc))` / `effect::get_label_object(d, e).as_card()` |
| `g:AddCard(c)` / `g:RemoveCard(c)`     | `group::add_card(&mut g, &CardSet::from_card(c))` / `group::remove_card(...)` |
| `g:FilterCount(f, nil)`                 | `group::filter_count(d, &g, &f, NO_EX)` (the `nil` is `ex`, not a filter argument; `filter_count`, `is_exists`, `filter` take the filter without `Option`) |
| `local g1,g2=g:Split(f, nil)`           | `let (g1, g2) = group::split(d, &g, &f, NO_EX)` (matching, the rest) |
| `Duel.SpecialSummonStep(c, ...)` / `Duel.SpecialSummonComplete()` | `duel::special_summon_step(d, c, sumtype, sp, tp, nocheck, nolimit, pos, None)` / `duel::special_summon_complete(d)` |
| `Duel.RegisterEffect(e, tp)`            | `duel::register_effect(d, e, tp)`                  |
| `e:SetLabelObject(g)` / `e:GetLabelObject()` (a group) | `effect::set_label_object(d, e, Value::Group(g))` / `effect::get_label_object(d, e).as_group().cloned()` (`.nil()` where the Lua indexes it) |
| `c:SetUniqueOnField(1, 0, function(c) ... end)` | `card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Function(lua_fn(f)), None)` (or `card::UniqueCode::Code(ID)`); `f` reads the card as `a.c(0)` |
| `aux.SelectUnselectGroup(g, e, tp, 1, 2, s.rescon, ...)` | `aux::select_unselect_group(...)` with `rescon: fn(d, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool)` (`(res, stop)`, see "Library") |
| `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0)` (a library cost called from a card) | `aux::bfgcost().call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), ...9 values]).truthy()` |

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
  `Duel.IsPlayerCanDraw(p)` without a count is `duel::is_player_can_draw(d, p, 0)` (not 1: the
  count only checks the Deck size). Always look the default up; never guess it.
  **An explicit `nil` is not a missing argument for booleans:** the C++ `lua_get<bool, def>` uses the
  default only when the argument is absent; `nil` (e.g. a variable that is nil) reads as `false`.
  For numbers, `nil` gives the default.
  `Duel.SelectOption(p, d1, d2)` is `duel::select_option(d, p, true, &[d1, d2])` (`sel_hint` defaults to
  true: the opponent sees the choice); `Duel.SelectOption(p, false, d1, d2)` passes `false`.
  `c:CancelToGrave()` is `card::cancel_to_grave(d, c, true)`.
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
- `e:SetCountLimit(n)` is `effect::set_count_limit(d, e, n, 0, 0, 0)`; `SetCountLimit(1, id)` is
  `(d, e, 1, ID, 0, 0)`; `SetCountLimit(1, {id, 1})` is `(d, e, 1, ID, 1, 0)`;
  `SetCountLimit(1, id, EFFECT_COUNT_CODE_OATH)` is `(d, e, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8)`.
- `e:SetHintTiming(a)` is `effect::set_hint_timing(d, e, a, None)`, `SetHintTiming(a, b)` is
  `(d, e, a, Some(b))`.
- `c:IsHasEffect(code)` returns a `Vec` (all effects): `not c:IsHasEffect(X)` is `.is_empty()`.
- Event codes: copy them exactly (`EVENT_PHASE_START|PHASE_STANDBY` is not `EVENT_PHASE|PHASE_STANDBY`).
- `local e2=e1:Clone()` is `let e2 = effect::clone(d, e1);`.
- `aux.SelectUnselectGroup`: a `rescon` returns `(valid, stop)`; a Lua rescon with one return value is
  `(res, false)`. With `chk == 0` the result is `matches!(.., Err(true))`, with `chk == 1` `.unwrap_or_default()`.
- `Duel.GetTargetCards(e)` is `duel::get_target_cards(d, e)` (an `Option`): `.nil()` where the Lua indexes it.
- Calling a library `LuaFn` from Rust (`aux.bdocon(e,tp,...)`, `Gemini.EffectStatusCondition(e)`):
  `f.call(d, &[Value::Effect(e), ...]).truthy()`.
- `c.rum_limit` (of an Extra Deck monster, read by Rank-Up-Magic cards) is
  `crate::card_info(d, c).rum_limit` (`Option<fn(d, mc, e) -> bool>`; `None` = the Lua `nil`). A
  card that defines `s.rum_limit=function(c,e) ... end` declares it in `INFO` as
  `rum_limit: Some(rum_limit)`. `s.rum_xyzsummon` is only read by unofficial scripts: leave it out.
- A script that sets `s.material_count`, `s.min_material_count`, `s.max_material_count` (or
  `s.material`) by hand inside `if c.material_count==nil then ... end` writes them with
  `crate::class::set(d, c, crate::class::MATERIAL_COUNT, Value::Int(n))` (and `MIN_MATERIAL_COUNT`,
  `MAX_MATERIAL_COUNT`) under `if crate::class::get(d, c, crate::class::MATERIAL_COUNT).is_nil()`,
  the same way the Fusion procedures do (see `fusion.rs`); a static `s.material={...}` outside that
  `if` goes in `INFO`.
- `c.material` (of any card) is `crate::class::material(d, c)`: procedures such as `Fusion.AddProcMix` set it at
  run time, so `card_info(d, c).material` (the static declaration only) is wrong.
- `c:Race(v)` / `c:Attribute(v)` / `c:Level()` ... (the printed data, `CARD_INFO_FUNC` in libcard.cpp) are
  `card::info_race(d, c, Some(v))` / `card::info_level(d, c, None)` and so on (`info_` + snake name). Only these bare names: `c:GetLevel()`, `c:GetAttack()`, ... are the current values (`card::get_level`, ...).
- `Duel.GetMetatable(code)` (another card's script table) is `crate::script_info(code)` for its declarations
  (`.lv`, `.listed_names`, ...); runtime fields of a card go through `crate::class::get(d, c, ...)`.
- `Duel.LoadCardScript("c980973.lua")` does nothing in Rust (all scripts are compiled in): leave it out and
  refer to the other card's module (`super::c980973::...`) where the Lua uses its table.
- A C function used as a filter (`Card.IsAbleToRemove` as a Fusion `matfilter`) gets the caller's extra
  arguments by position: with `(c, e, tp, chk)` the effect is not a number (default player), `tp` becomes
  the position and `chk` the reason: `card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))`.
- `e:GetChainData().x = v` / `e:GetChainData().x` (a field the card invents) is
  `chain::set_effect_data(d, e, None, chain::field::CARD + n, v)` / `chain::effect_data(d, e, None, chain::field::CARD + n)`,
  with one `n` (0, 1, ...) per field name; the library fields are `chain::field::COST_*`.
- `Group.Select` / `Duel.SelectMatchingCard` / `Duel.SelectTarget` are cancelable only when the argument at the
  cancelable position is a boolean; extra arguments after `ex` go to the filter. They still return `Option`:
  `.nil()` before using the group.
- `Duel.Equip(p, c, tc)` is `duel::equip(d, p, c, tc, true, false)`.
- `Duel.CheckReleaseGroupCost(tp, f, minc, maxc, use_hand, check, ex, ...)` /
  `SelectReleaseGroupCost` are `duel::check_release_group_cost(d, tp, f, minc, maxc, use_hand, check, ex)`. The
  old form without `maxc` (the 4th argument is a boolean: `(tp, f, minc, use_hand, check, ex, x, y)`) means
  `maxc = minc`; the 5th argument is `check`, the 6th `ex`, and the 7th and later go to the filter AND to the check (proc_workaround.lua passes them to both)
  (capture them in the closure). See `Duel.CheckReleaseGroupCost` in proc_workaround.lua.
- `aux.TRUE` as the filter of `Duel.CheckReleaseGroup`/`SelectReleaseGroup` (and the `...Cost`/`...Ex`
  forms) is `Some(&|_, _| true)`, NOT `None`: with a filter function the core leaves out face-down
  opponent monsters, with `nil` it does not. Everywhere else `aux.TRUE` and `nil` are the same.
- `aux.TRUE` where a function takes a plain `Filter` (`group::filter_select`, `group::is_exists`) is `&|_, _| true`.
- `g:KeepAlive()` and `g:DeleteGroup()` do nothing in Rust (groups are values): leave them out.
- A Lua helper function the script never calls can be left out.
- `Duel.SetOperationInfo(0, cat, 0, ...)` (the number 0 as the cards) is the same as `nil`: `None`.
- `Cost.Discard()` is `cost::discard(None, false, None, None, None)`; a filter is
  `Some(std::sync::Arc::new(|d, c, e, tp| ...))`.
- Functions that receive `c` (`EFFECT_SPSUMMON_PROC` condition `function(e,c)`, its target/operation)
  are `lua_fn(f)` with `f(d, a: Args)`, reading the arguments **by position, exactly as the core passes
  them**: condition `(e, c, ...)`: `a.e(0)`, `a.card(1)` (nil when the core only checks the effect);
  **target `(e,tp,eg,ep,ev,re,r,rp,chk,c)`: index 8 is `chk` (the number 1), the card is index 9**;
  operation `(e,tp,eg,ep,ev,re,r,rp,c)`: the card is index 8. A Lua target written as
  `function(e,tp,eg,ep,ev,re,r,rp,c)` (no `chk` parameter) gets the NUMBER 1 as its `c`: passing it on
  to a C function that wants a card gives nil there (`a.card(8)` → `None`), and indexing it
  (`c:GetControler()`) is a Lua error (`a.c(8)` raises it).
- **Lua `/` is float division.** `c:GetAttack()/2` is `3.5` for an odd 7. Where it reaches the core
  (`SetValue`, a value function's result, `Duel.Damage`, ...), the core rounds half away from zero:
  `lua_int(atk as f64 / 2.0)`. Keep comparisons in `f64` (`atk as f64 / 2.0 > x as f64`).
  `math.floor(a/b)` and `a//b` are integer floor division (`a.div_euclid(b)` for `b > 0`).
- A value function that falls off the end returns `nil` in Lua, which the core reads as `0`.
  A value function returning `false` is also read as `0`. A Lua number is always true in a condition:
  `if Duel.SendtoGrave(...) and ...` / `Duel.SpecialSummon(...)` / `GetLocationCountFromEx(...)` used
  as a boolean are true even when they return 0 (call them, then continue as true; never `> 0`).
- An operation whose return value the core reads (the operation of a `*_REPLACE` effect such as
  `EFFECT_OVERLAY_REMOVE_REPLACE`: `return true` = replaced) must return it: write it as `condition(f)` (a
  `bool`), not `operation(f)`.
- Core functions that take `&Duel` (`card::is_faceup`, ...) cannot be passed as a filter directly:
  write `Some(&|d, c| card::is_faceup(d, c))`.
- A one-argument `Duel.HintSelection(g)` is `duel::hint_selection(d, g, true)` (the default is
  `true`).
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
- `Duel.Remove(g, nil, reason)` / `Duel.Remove(g, 0, reason)`: position `0`, which keeps the card's
  current position (face-up Attack stays face-up Attack). Only an explicit `POS_FACEUP` is `POS_FACEUP`.
- Defaults that often get guessed wrong: `Duel.Overlay(xyz, mats)` does not send to the GY
  (`send_to_grave` false); `Duel.DisableShuffleCheck()` is `true`; `g:Select` is not cancelable.
  A `CHAININFO_*` value beyond the chain is `Value::Nil`: compare it as `None`, not as 0.
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

A script that calls a function that exists neither in the C++ (`build/ocgcore-src/lib*.cpp`) nor in
the Lua libraries (e.g. `c:IsInMainZone()`) or a field that no script defines raises "attempt to
call a nil value" when that line runs: write `crate::lua_error("attempt to call a nil value")` at
that point (keep everything before it). The same holds for a Lua global that is never assigned
(`chkc` in a function without a `chkc` parameter): it is `nil`, not an error, until something
indexes or calls it. `not x==y` is `(not x)==y` in Lua.

## State

**All state must live in the duel**, never in Rust `static`s, `thread_local`s, or variables
captured by closures and changed later. The engine copies duels (that is the point of this port),
and state kept elsewhere would not be copied.

- Lua global variables (`_replace_count=0`, shared by several scripts) are `lua_global(d, "_replace_count")` and
  `set_lua_global(d, "_replace_count", Value::Int(0))`.
- Lua `s.x` / `s[0]` (variables on the card table) become `s_var(d, ID, key)` and
  `set_s_var(d, ID, key, Value::Int(v))`, where `key` is a number you choose per variable.
- `aux.GlobalCheck(s, function() ... end)` becomes `aux::global_check(d, ID, |d| { ... })`.
- Effect labels (`e:SetLabel`, `e:SetLabelObject`) are already state in the duel. Use them as the
  Lua does.
- A group that several effects share and change (Lua: one `Group.CreateGroup()` with `KeepAlive`,
  set as the label object of two effects, then `AddCard`/`Clear` on it): keep it in ONE place, e.g.
  the label object of the first effect as `Value::Group(g)`; the other effects reach it through
  their own label object `Value::Effect(e_first)`, and every change is stored back with
  `effect::set_label_object(d, e_first, Value::Group(g))`. Never use `d.new_group` /
  `Value::SharedGroup` in a card: those groups belong to the core and are freed between units.
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
