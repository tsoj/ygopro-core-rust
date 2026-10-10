# Rust duel engine: design

A port of edo9300's ygopro-core (with our patch, `build/ocgcore-src`) and of the card scripts
(`external/CardScripts`) to Rust. Goals, in order:

1. **Same behavior as ygopro-core.** The C++ core is the oracle: the same seed, decks and responses
   must give the same message stream (byte for byte) and the same query results. Differential tests
   (`ygo-diff`) run both engines side by side and report the first message that differs.
2. **Cheap copies** (the main reason for the port). A duel is plain data (`#[derive(Clone)]`):
   cards, effects and groups are indices into vectors, never pointers; card functions are `Arc`
   closures (immutable, shared by copies). Cards and effects are copy-on-write (`Vec<Arc<Card>>`,
   `Vec<Option<Arc<Effect>>>`; `card_mut`/`effect_mut` copy one only while another duel still
   shares it), so a copy costs a reference count per card and effect. Mutable state of card scripts
   (Lua upvalues and globals) lives in the duel, never in a closure, so it is copied too.
   Measured with `ygo-diff --bench-clone`: a vanilla duel (81 cards) copies in 1.8 us.
3. **Speed.** No interpreter, no process forks to copy a state.

## Faithful port

The C++ is ported function by function, keeping names and structure (`field::destroy` becomes
`Duel::destroy`, `Processors::Destroy` with its `step` numbers becomes `Unit::Destroy`), so a
difference can be traced back to the C++ line it came from. Rules are not redesigned.

- `card*` -> `CardId` (index into `Duel::cards`, equal to the core's `cardid` minus one).
  `card_set` (a std::set ordered by cardid) -> `CardSet`, a sorted `Vec<CardId>`.
- `effect*` -> `EffectId` (index into `Duel::effects`; a slot is `None` once deleted).
  `std::multimap<uint32_t, effect*>` -> a `Vec<(u32, EffectId)>` kept sorted by key, equal keys in
  insertion order (same iteration order as the multimap).
- `group*` -> `Group`, a value (sorted `Vec<CardId>`). Lua groups are shared mutable objects; the
  few scripts that rely on aliasing are translated to keep the group in effect labels instead.
- Lua function references -> `LuaFn` (`Arc<dyn Fn(&mut Duel, &[Param]) -> Ret>`). The core calls
  them like the interpreter does: a parameter list (`add_param`) and a result.
- All C++ containers that are iterated are checked for order: `unordered_*` iteration only where
  the result does not depend on the order.

## Card code that waits for a player: replay

In ygopro-core, card costs, targets and operations run as Lua coroutines: `Duel.SelectMatchingCard`
or `Duel.Destroy` yield, the core processes the pushed units (asking players if needed), and the
coroutine resumes. Rust card code is ordinary straight-line code, so instead:

- Running a card's cost/target/operation (`ExecuteCost/Target/Operation`) opens a **replay scope**:
  a clone of the duel taken before the call, plus the list of responses given inside it so far.
- Inside the scope, core operations that the Lua API runs synchronously (yielding) are processed to
  completion by a nested unit loop; a question takes the next response from the list.
- A question with no response left aborts the call (`Err(Interrupt)`): the duel is restored from the
  clone, the question is sent, and the scope waits. When the response arrives, the call runs again
  from the start with one more response. Messages already sent are not sent again (the scope counts
  them). Everything is deterministic (the RNG is part of the clone), so the run repeats exactly.
- While waiting, queries see the state as it was at the question (kept separately).

A waiting duel is therefore still plain data and can be cloned at any decision.

## Layout

- `ygo-core`: the engine (duel state, rules, the card-facing API that replaces libduel/libcard/
  libgroup/libeffect, the procedures of utility.lua), and an `ocgapi`-compatible C ABI so the
  Python tools can drive it (`YGOAI_CORE_LIB`), plus `OCG_DuelClone`.
- `ygo-cards`: card implementations, one function per card (translations of the Lua scripts).
- `ygo-diff`: differential tester; loads `build/libocgcore.so` (C++) next to the Rust engine.

## Plan

1. Core: data types, duel/effect/card/field, operations, processor, player operations, queries.
   Milestone: random duels with vanilla decks give identical message streams. Measure clone speed.
2. Card API and procedures (Synchro, Xyz, Fusion, Link, Ritual, Pendulum, ...).
3. Cards, most played first, each gated by differential tests (subagents: Haiku, then Sonnet, then Opus).

License: AGPL-3.0-or-later, like ygopro-core and the card scripts it is ported from.
