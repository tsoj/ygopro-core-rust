//! The API of the core: `ocg` is `ocgapi.cpp` (creating and running duels); the others are the
//! functions card scripts use (`libeffect.cpp`, `libcard.cpp`, `libgroup.cpp`, `libduel.cpp`,
//! `libdebug.cpp`), as free functions named after the Lua ones: `e:SetType(v)` is
//! `effect::set_type(d, e, v)`, `Duel.Destroy(g, r)` is `duel::destroy(d, g, r, ..)`.

pub mod card;
pub mod effect;
pub mod ocg;

pub use ocg::{NewCardInfo, QueryInfo};
pub mod debug;
pub mod duel;
pub mod group;

use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use crate::duel::Duel;
use crate::lua::LuaError;
use crate::types::*;

/// A card filter of a script (`function(c, ...)`: the extra arguments are captured).
pub type Filter<'a> = &'a dyn Fn(&mut Duel, CardId) -> bool;
/// A value function of a script (`function(c, ...)` returning a number).
pub type ValueFn<'a> = &'a dyn Fn(&mut Duel, CardId) -> i64;

/// The card or group to leave out (`ex` of the Lua API: nil, a card or a group).
#[derive(Clone, Copy)]
pub enum Exclude<'a> {
    None,
    Card(CardId),
    Group(&'a CardSet),
}
/// No exclusion (Lua `nil`).
pub const NO_EX: Exclude<'static> = Exclude::None;

impl From<CardId> for Exclude<'_> {
    fn from(c: CardId) -> Self {
        Exclude::Card(c)
    }
}
impl From<Option<CardId>> for Exclude<'_> {
    fn from(c: Option<CardId>) -> Self {
        c.map_or(Exclude::None, Exclude::Card)
    }
}
impl<'a> From<&'a CardSet> for Exclude<'a> {
    fn from(g: &'a CardSet) -> Self {
        Exclude::Group(g)
    }
}
impl<'a> From<Option<&'a CardSet>> for Exclude<'a> {
    fn from(g: Option<&'a CardSet>) -> Self {
        g.map_or(Exclude::None, Exclude::Group)
    }
}

impl Exclude<'_> {
    /// Removes the excluded cards from `cset`.
    pub fn remove_from(&self, cset: &mut CardSet) {
        match self {
            Exclude::None => {}
            Exclude::Card(c) => {
                cset.remove(*c);
            }
            Exclude::Group(g) => {
                for c in g.iter() {
                    cset.remove(c);
                }
            }
        }
    }
    pub fn contains(&self, c: CardId) -> bool {
        match self {
            Exclude::None => false,
            Exclude::Card(x) => *x == c,
            Exclude::Group(g) => g.contains(c),
        }
    }
}

/// The core (build/ocgcore-src interpreter.cpp) stops a process call that runs more than 1e9 Lua
/// instructions with a script error (scripts can take exponential time). Lua instructions are not
/// counted here; the guarded script callbacks are, against a limit that is reached later than the
/// core's (a callback takes well over the 33 Lua instructions per callback this allows for), so
/// that a step the core finishes is never cut short, and a step it does not finish ends too.
pub const SCRIPT_STEP_BUDGET: u64 = 30_000_000;

/// Counts a script callback against `SCRIPT_STEP_BUDGET`; past it, every callback is a Lua error
/// until the next `ocg_process` (as the core's budget hook).
#[inline]
pub fn script_step(d: &mut Duel) {
    d.script_steps += 1;
    if d.script_steps > SCRIPT_STEP_BUDGET {
        crate::lua::lua_error("script instruction budget exceeded");
    }
}

/// Runs a script callback; a Lua error in it is logged and gives `default` (as the core's
/// `check_matching`/`get_operation_value`).
pub fn protected<T>(d: &mut Duel, default: T, f: impl FnOnce(&mut Duel) -> T) -> T {
    match catch_unwind(AssertUnwindSafe(|| {
        script_step(d);
        f(d)
    })) {
        Ok(v) => v,
        Err(payload) => match payload.downcast::<LuaError>() {
            Ok(e) => {
                d.handle_message(&e.0, 0);
                default
            }
            Err(other) => resume_unwind(other),
        },
    }
}
/// `interpreter::check_matching`
pub fn check_matching(d: &mut Duel, f: Filter, c: CardId) -> bool {
    check_matching_with(d, c, |d| f(d, c))
}
/// `check_matching` with the filter as a closure: for library code that runs a Lua helper as a
/// card filter of a group function (e.g. `g:Filter(aux.SelectUnselectLoop, ...)`), which the
/// belief replay's forced matches and read log apply to as to any filter.
pub fn check_matching_with(d: &mut Duel, c: CardId, f: impl FnOnce(&mut Duel) -> bool) -> bool {
    // (ext: a card forced to match, or not, decides the filters of card scripts, not those of the
    // rule helpers (`Ext::rule_filter`, for this filter only: the filters it uses itself are not)
    let rule = std::mem::replace(&mut d.ext.rule_filter, false);
    if !rule {
        if let Some(forced) = d.ext_forced_matching(c) {
            d.ext.rule_filter = rule;
            return forced;
        }
    }
    let result = protected(d, false, f);
    if !rule {
        d.ext_log_filter_read(c, result);
    }
    d.ext.rule_filter = rule;
    result
}

/// Runs `f` with `Ext::rule_filter` set: the card filters it passes directly to the engine's group
/// functions are those of a rule helper (`proc_workaround.lua`), which forced matches and the read
/// log of the belief replay do not apply to.
pub fn rule_filters<T>(d: &mut Duel, f: impl FnOnce(&mut Duel) -> T) -> T {
    let old = std::mem::replace(&mut d.ext.rule_filter, true);
    let r = f(d);
    d.ext.rule_filter = old;
    r
}
/// `interpreter::get_operation_value`
pub fn get_operation_value(d: &mut Duel, f: ValueFn, c: CardId) -> i64 {
    protected(d, 0, |d| f(d, c))
}
/// `push_return_cards`: the selected cards, `None` if the selection was canceled and cancelable.
pub fn return_cards(d: &Duel, cancelable: bool) -> Option<CardSet> {
    if d.return_cards.canceled {
        if cancelable { None } else { Some(CardSet::new()) }
    } else {
        Some(CardSet::from_iter(d.return_cards.list.iter().copied()))
    }
}

/// A card or a group, where the Lua function treats them differently.
#[derive(Clone)]
pub enum CardOrGroup {
    Card(CardId),
    Group(CardSet),
}
impl From<CardId> for CardOrGroup {
    fn from(c: CardId) -> Self {
        CardOrGroup::Card(c)
    }
}
impl From<CardSet> for CardOrGroup {
    fn from(g: CardSet) -> Self {
        CardOrGroup::Group(g)
    }
}
impl From<&CardSet> for CardOrGroup {
    fn from(g: &CardSet) -> Self {
        CardOrGroup::Group(g.clone())
    }
}
impl CardOrGroup {
    pub fn cards(&self) -> CardSet {
        match self {
            CardOrGroup::Card(c) => CardSet::from_card(*c),
            CardOrGroup::Group(g) => g.clone(),
        }
    }
}
