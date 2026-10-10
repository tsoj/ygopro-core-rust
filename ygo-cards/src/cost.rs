//! `Cost.*` (`utility.lua`): common costs. The global scripts record some costs in tables keyed by
//! the function (`cost_tables`): here the function's `tag` (`TAG_*`).

use std::sync::{Arc, LazyLock};

use crate::chain;
use crate::constants::*;
use crate::lua::{card, duel, effect};
use crate::prelude::*;

pub const TAG_SELF_TOGRAVE: u32 = 1;
pub const TAG_SELF_DISCARD: u32 = 2;
pub const TAG_DETACH: u32 = 4;
pub const TAG_SELF_CHANGEPOS: u32 = 8;
pub const TAG_REMAIN_FIELD: u32 = 16;

fn self_cost(check: fn(&mut Duel, CardId) -> bool, pay: fn(&mut Duel, CardId, u8)) -> LuaFn {
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if a.chk == Some(0) {
            return check(d, c);
        }
        pay(d, c, a.tp);
        false
    })
}

macro_rules! shared {
    ($(#[$m:meta])* $name:ident, $static:ident, $e:expr) => {
        static $static: LazyLock<LuaFn> = LazyLock::new(|| $e);
        $(#[$m])*
        pub fn $name() -> LuaFn {
            $static.clone()
        }
    };
}

shared!(
    /// `Cost.SelfBanish` (`aux.bfgcost`)
    self_banish, SELF_BANISH,
    self_cost(|d, c| card::is_able_to_remove_as_cost(d, c, None), |d, c, _| {
        duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    })
);
shared!(
    /// `Cost.SelfTribute` (`Cost.SelfRelease`)
    self_tribute, SELF_TRIBUTE,
    self_cost(|d, c| card::is_releasable(d, c, None, None), |d, c, _| {
        duel::release(d, c, REASON_COST, None);
    })
);
shared!(
    /// `Cost.SelfToGrave`
    self_to_grave, SELF_TO_GRAVE,
    self_cost(|d, c| card::is_able_to_grave_as_cost(d, c), |d, c, _| {
        duel::send_to_grave(d, c, REASON_COST, None, None);
    })
    .with_tag(TAG_SELF_TOGRAVE)
);
shared!(
    /// `Cost.SelfToHand`
    self_to_hand, SELF_TO_HAND,
    self_cost(|d, c| card::is_able_to_hand_as_cost(d, c), |d, c, _| {
        duel::send_to_hand(d, c, None, REASON_COST, None);
    })
);
shared!(
    /// `Cost.SelfToDeck`
    self_to_deck, SELF_TO_DECK,
    self_cost(|d, c| card::is_able_to_deck_as_cost(d, c), |d, c, _| {
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    })
);
shared!(
    /// `Cost.SelfToExtra`
    self_to_extra, SELF_TO_EXTRA,
    self_cost(|d, c| card::is_able_to_extra_as_cost(d, c), |d, c, _| {
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    })
);
shared!(
    /// `Cost.SelfReveal`
    self_reveal, SELF_REVEAL,
    self_cost(|d, c| !card::is_public(d, c), |d, c, tp| {
        duel::confirm_cards(d, 1 - tp, c);
    })
);
shared!(
    /// `Cost.SelfDiscard`
    self_discard, SELF_DISCARD,
    self_cost(|d, c| card::is_discardable(d, c, None, None), |d, c, _| {
        duel::send_to_grave(d, c, REASON_DISCARD | REASON_COST, None, None);
    })
    .with_tag(TAG_SELF_DISCARD)
);
shared!(
    /// `Cost.SelfDiscardToGrave`
    self_discard_to_grave, SELF_DISCARD_TO_GRAVE,
    self_cost(|d, c| card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c), |d, c, _| {
        duel::send_to_grave(d, c, REASON_DISCARD | REASON_COST, None, None);
    })
    .with_tag(TAG_SELF_TOGRAVE | TAG_SELF_DISCARD)
);
shared!(
    /// `Cost.HintSelectedEffect`
    hint_selected_effect, HINT_SELECTED_EFFECT,
    cost_target(|d, a| {
        if a.chk == Some(0) {
            return true;
        }
        let desc = effect::get_description(d, a.e);
        duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
        false
    })
);
shared!(
    /// `aux.RemainFieldCost`: cards that can stay on the field, but not always
    remain_field_cost, REMAIN_FIELD_COST,
    cost_target(|d, a| {
        if a.chk == Some(0) {
            return true;
        }
        let c = effect::get_handler(d, a.e).unwrap();
        let cid = chain::get_id(d, 0);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_REMAIN_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
        effect::set_reset(d, e1, RESET_CHAIN, 0);
        card::register_effect(d, c, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EVENT_CHAIN_DISABLED);
        effect::set_operation(d, e2, Some(remain_field_disabled()));
        effect::set_label(d, e2, &[cid]);
        effect::set_reset(d, e2, RESET_CHAIN, 0);
        duel::register_effect(d, e2, a.tp);
        false
    })
    .with_tag(TAG_REMAIN_FIELD)
);
shared!(
    /// `aux.RemainFieldDisabled`
    remain_field_disabled, REMAIN_FIELD_DISABLED,
    operation(|d, a| {
        let cid = chain::get_id(d, a.ev as u8);
        if cid != effect::get_label(d, a.e) {
            return;
        }
        let owner = effect::get_owner(d, a.e).unwrap();
        if card::is_location(d, owner, LOCATION_ONFIELD) {
            card::cancel_to_grave(d, owner, false);
        }
    })
);

/// `Cost.RemoveCounterFromSelf(counter_type, count)`
pub fn remove_counter_from_self(counter_type: u16, count: u16) -> LuaFn {
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if a.chk == Some(0) {
            return card::is_can_remove_counter(d, c, a.tp, counter_type, count, REASON_COST);
        }
        card::remove_counter(d, c, a.tp, counter_type, count, REASON_COST);
        false
    })
}
/// `Cost.RemoveCounterFromField(counter_type, count)`
pub fn remove_counter_from_field(counter_type: u16, count: u32) -> LuaFn {
    cost_target(move |d, a| {
        if a.chk == Some(0) {
            return duel::is_can_remove_counter(d, a.tp, 1, 0, counter_type, count, REASON_COST);
        }
        duel::remove_counter(d, a.tp, 1, 0, counter_type, count, REASON_COST);
        false
    })
}
/// `Cost.SelfChangePosition(position)`
pub fn self_change_position(position: u32) -> LuaFn {
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if a.chk == Some(0) {
            return card::is_can_change_position(d, c) && !card::is_position(d, c, position) && (position & POS_FACEDOWN == 0 || card::is_can_turn_set(d, c));
        }
        let fd_chk = card::is_position(d, c, POS_FACEDOWN) && position & POS_FACEUP > 0;
        duel::change_position(d, c, position as u8, None, None, None, false);
        if fd_chk {
            card::set_status(d, c, STATUS_EFFECT_ENABLED, true);
        }
        false
    })
    .with_tag(TAG_SELF_CHANGEPOS)
}

/// A count of `Cost.Discard`/`Reveal`/`DetachFromSelf`: a number, or a function of `(e, tp)`.
#[derive(Clone)]
pub enum Count {
    N(i64),
    F(Arc<dyn Fn(&mut Duel, EffectId, u8) -> i64 + Send + Sync>),
}
impl Count {
    fn get(&self, d: &mut Duel, e: EffectId, tp: u8) -> i64 {
        match self {
            Count::N(n) => *n,
            Count::F(f) => f(d, e, tp),
        }
    }
}
/// A card filter of a cost: `function(c, e, tp)`.
pub type CostFilter = Arc<dyn Fn(&mut Duel, CardId, EffectId, u8) -> bool + Send + Sync>;
/// What a cost does with the cards afterwards: `function(e, tp, g)`.
pub type CostOp = Arc<dyn Fn(&mut Duel, EffectId, u8, &CardSet) + Send + Sync>;

/// `Cost.Discard(filter, other, min = 1, max = min, op)`
pub fn discard(filter: Option<CostFilter>, other: bool, min: Option<Count>, max: Option<Count>, op: Option<CostOp>) -> LuaFn {
    cost_target(move |d, a| {
        let e = a.e;
        let tp = a.tp;
        let min_count = min.as_ref().map_or(1, |m| m.get(d, e, tp));
        let max_count = max.as_ref().map_or(min_count, |m| m.get(d, e, tp));
        let exclude: crate::lua::Exclude = if other { effect::get_handler(d, e).into() } else { crate::lua::NO_EX };
        let filter = filter.clone();
        let filter_final = move |d: &mut Duel, c: CardId| filter.as_ref().is_none_or(|f| f(d, c, e, tp)) && card::is_discardable(d, c, None, None);
        if a.chk == Some(0) {
            return min_count > 0 && max_count >= min_count && duel::is_existing_matching_card(d, Some(&filter_final), tp, LOCATION_HAND, 0, min_count as u32, exclude);
        }
        duel::discard_hand(d, tp, Some(&filter_final), min_count as u16, max_count as u16, REASON_COST | REASON_DISCARD, exclude);
        let g = duel::get_operated_group(d);
        chain::set_effect_data(d, e, None, chain::field::COST_DISCARDED_CARDS, Value::Group(g.clone()));
        if let Some(op) = &op {
            op(d, e, tp, &g);
        }
        false
    })
}
/// `Cost.Reveal(filter, other, min = 1, max = min, op, location = LOCATION_HAND)`
pub fn reveal(filter: Option<CostFilter>, other: bool, min: Option<Count>, max: Option<Count>, op: Option<CostOp>, location: Option<u32>) -> LuaFn {
    let location = location.unwrap_or(LOCATION_HAND);
    cost_target(move |d, a| {
        let e = a.e;
        let tp = a.tp;
        let min_count = min.as_ref().map_or(1, |m| m.get(d, e, tp));
        let max_count = max.as_ref().map_or(min_count, |m| m.get(d, e, tp));
        let exclude: crate::lua::Exclude = if other { effect::get_handler(d, e).into() } else { crate::lua::NO_EX };
        let filter = filter.clone();
        let filter_final = move |d: &mut Duel, c: CardId| filter.as_ref().is_none_or(|f| f(d, c, e, tp)) && !card::is_public(d, c);
        if a.chk == Some(0) {
            return min_count > 0 && max_count >= min_count && duel::is_existing_matching_card(d, Some(&filter_final), tp, location, 0, min_count as u32, exclude);
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        let g = duel::select_matching_card(d, tp, Some(&filter_final), tp, location, 0, min_count as u16, max_count as u16, false, exclude).unwrap_or_default();
        duel::confirm_cards(d, 1 - tp, &g);
        let in_loc = |d: &mut Duel, l: u32| crate::lua::group::is_exists(d, &g, &|d, c| card::is_location(d, c, l), 1, crate::lua::NO_EX);
        if in_loc(d, LOCATION_HAND) {
            duel::shuffle_hand(d, tp);
        }
        if in_loc(d, LOCATION_DECK) {
            duel::shuffle_deck(d, tp);
        }
        if in_loc(d, LOCATION_EXTRA) {
            duel::shuffle_extra(d, tp);
        }
        if let Some(op) = &op {
            op(d, e, tp, &g);
        }
        false
    })
}
/// What a detach cost does with the detached materials: `function(e, g)`.
pub type DetachOp = Arc<dyn Fn(&mut Duel, EffectId, &CardSet) + Send + Sync>;

fn after_detach(d: &mut Duel, e: EffectId, op: &Option<DetachOp>) {
    let g = duel::get_operated_group(d);
    chain::set_effect_data(d, e, None, chain::field::COST_DETACHED_MATERIALS, Value::Group(g.clone()));
    if let Some(op) = op {
        op(d, e, &g);
    }
}
/// `Cost.DetachFromSelf(min, max = min, op)`
pub fn detach_from_self(min: Count, max: Option<Count>, op: Option<DetachOp>) -> LuaFn {
    let max = max.unwrap_or_else(|| min.clone());
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        let min_count = min.get(d, a.e, a.tp);
        let max_count = max.get(d, a.e, a.tp);
        if a.chk == Some(0) {
            return min_count > 0 && max_count >= min_count && card::check_remove_overlay_card(d, c, a.tp, min_count as u16, REASON_COST);
        }
        if card::remove_overlay_card(d, c, a.tp, min_count as u16, max_count as u16, REASON_COST) > 0 {
            after_detach(d, a.e, &op);
        }
        false
    })
    .with_tag(TAG_DETACH)
}
/// `Cost.DetachChoiceFromSelf(choices, op)`: choices are the possible numbers (or a function of
/// `(e, tp)` giving them).
pub fn detach_choice_from_self(choices: Arc<dyn Fn(&mut Duel, EffectId, u8) -> Vec<i64> + Send + Sync>, op: Option<DetachOp>) -> LuaFn {
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        let final_choices: Vec<i64> = choices(d, a.e, a.tp).into_iter().filter(|&ct| card::check_remove_overlay_card(d, c, a.tp, ct as u16, REASON_COST)).collect();
        if a.chk == Some(0) {
            return !final_choices.is_empty();
        }
        let mut amt = final_choices[0];
        if final_choices.len() > 1 {
            let nums: Vec<u64> = final_choices.iter().map(|&x| x as u64).collect();
            amt = duel::announce_number(d, a.tp, &nums).map_or(0, |x| x.0 as i64);
        }
        if card::remove_overlay_card(d, c, a.tp, amt as u16, amt as u16, REASON_COST) > 0 {
            after_detach(d, a.e, &op);
        }
        false
    })
    .with_tag(TAG_DETACH)
}
/// The amount `Cost.PayLP` pays.
#[derive(Clone, Copy)]
pub enum PayLp {
    /// X LP (X >= 1)
    Amount(i64),
    /// a fraction of the LP (half, a third, ...)
    Fraction(f64),
    /// so that X LP are left
    Until(i64),
}
/// `Cost.PayLP(lp_value, pay_until)`
pub fn pay_lp(v: PayLp) -> LuaFn {
    cost_target(move |d, a| {
        let paid = match v {
            PayLp::Amount(x) => {
                if a.chk == Some(0) {
                    return duel::check_lp_cost(d, a.tp, x as u32);
                }
                x
            }
            PayLp::Fraction(f) => {
                if a.chk == Some(0) {
                    return true;
                }
                (duel::get_lp(d, a.tp) as f64 * f).floor() as i64
            }
            PayLp::Until(x) => {
                let p = (duel::get_lp(d, a.tp) as i64 - x) as f64;
                let p = p.floor() as i64;
                if a.chk == Some(0) {
                    return p > 0 && duel::check_lp_cost(d, a.tp, p as u32);
                }
                p
            }
        };
        duel::pay_lp_cost(d, a.tp, paid as u32);
        chain::set_effect_data(d, a.e, None, chain::field::COST_LP_PAID, Value::Int(paid));
        false
    })
}
fn use_limit_cost(reset: u32, soft: bool, flag: u32, ct: Option<usize>) -> LuaFn {
    let ct = ct.unwrap_or(1);
    cost_target(move |d, a| {
        let c = effect::get_handler(d, a.e).unwrap();
        if a.chk == Some(0) {
            return (soft && !card::has_flag_effect(d, c, flag, Some(ct))) || (!soft && !duel::has_flag_effect(d, a.tp, flag, Some(ct)));
        }
        if soft {
            card::register_flag_effect(d, c, flag, RESET_EVENT | RESETS_STANDARD | reset, 0, 1, 0, 0);
        } else {
            duel::register_flag_effect(d, a.tp, flag, reset, 0, 1, 0);
        }
        false
    })
}
/// `Cost.SoftUseLimitPerChain(flag, ct)` (`Cost.SoftOncePerChain`)
pub fn soft_use_limit_per_chain(flag: u32, ct: Option<usize>) -> LuaFn {
    use_limit_cost(RESET_CHAIN, true, flag, ct)
}
pub fn soft_use_limit_per_battle(flag: u32, ct: Option<usize>) -> LuaFn {
    use_limit_cost(RESET_PHASE | PHASE_DAMAGE, true, flag, ct)
}
pub fn hard_use_limit_per_chain(flag: u32, ct: Option<usize>) -> LuaFn {
    use_limit_cost(RESET_CHAIN, false, flag, ct)
}
pub fn hard_use_limit_per_battle(flag: u32, ct: Option<usize>) -> LuaFn {
    use_limit_cost(RESET_PHASE | PHASE_DAMAGE, false, flag, ct)
}

fn args_with_chk(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}

/// `Cost.AND(costs...)`: all of them (recorded as each kind any of them is).
pub fn and(fns: Vec<LuaFn>) -> LuaFn {
    let tag = fns.iter().fold(0, |t, f| t | f.tag);
    LuaFn::new(move |d, args| {
        let a = EffArgs::with_chk(args);
        // when checking, stop at the first falsy value
        if a.chk == Some(0) {
            for f in &fns {
                if !f.call(d, &args_with_chk(&a, 0)).truthy() {
                    return Value::Bool(false);
                }
            }
            return Value::Bool(true);
        }
        // when executing, stop if a function returns 'false' specifically
        for f in &fns {
            if matches!(f.call(d, &args_with_chk(&a, 1)), Value::Bool(false)) {
                return Value::Bool(false);
            }
        }
        Value::Nil
    })
    .with_tag(tag)
}
/// One choice of `Cost.Choice`: the cost, its description, an additional check.
#[derive(Clone)]
pub struct CostChoice {
    pub cost: LuaFn,
    pub desc: u64,
    pub check: Option<LuaFn>,
}
/// `Cost.Choice({cost, desc, check}...)` (recorded as a kind when all choices are)
pub fn choice(choices: Vec<CostChoice>) -> LuaFn {
    let tag = choices.iter().fold(u32::MAX, |t, c| t & c.cost.tag);
    let tag = if choices.is_empty() { 0 } else { tag };
    LuaFn::new(move |d, args| {
        let a = EffArgs::with_chk(args);
        let mut ops = Vec::new();
        let mut has_choice = false;
        for ch in &choices {
            let check = ch.cost.call(d, &args_with_chk(&a, 0)).truthy() && ch.check.as_ref().is_none_or(|f| f.call(d, &args_with_chk(&a, 0)).truthy());
            ops.push((check, ch.desc));
            has_choice = has_choice || check;
        }
        if a.chk == Some(0) {
            return Value::Bool(has_choice);
        }
        let sel = crate::lua::duel::select_effect(d, a.tp, &ops);
        chain::set_effect_data(d, a.e, None, chain::field::COST_CHOICE, sel.map_or(Value::Nil, |s| Value::Int(s as i64)));
        // (Lua: choices[nil] is an error when nothing can be chosen)
        let Some(s) = sel else { crate::lua_error("attempt to index a nil value") };
        choices[s - 1].cost.call(d, &args_with_chk(&a, 1));
        Value::Nil
    })
    .with_tag(tag)
}
fn check_cost_replace_effect(d: &mut Duel, eff: EffectId, extracon: &LuaFn, a: &EffArgs) -> bool {
    if !effect::check_count_limit(d, eff, a.tp) {
        return false;
    }
    match effect::get_value(d, eff) {
        Value::Int(1) => true,
        Value::Fn(f) => {
            // (eff, extracon, e, tp, eg, ep, ev, re, r, rp, chk)
            let mut full = vec![Value::Effect(eff), Value::Fn(extracon.clone()), Value::Effect(a.e), Value::Int(a.tp as i64)];
            full.extend(args_with_chk(a, a.chk.unwrap_or(0)).into_iter().skip(2));
            f.call(d, &full).truthy()
        }
        _ => false,
    }
}
fn select_cost_replace_effect(d: &mut Duel, tp: u8, effs: &[EffectId]) -> EffectId {
    if effs.len() == 1 {
        return effs[0];
    }
    let descs: Vec<u64> = effs.iter().map(|&e| effect::get_description(d, e)).collect();
    let op = duel::select_option(d, tp, false, &descs);
    effs[op as usize]
}
/// The EFFECT_COST_REPLACE effect to apply instead of the base cost (`None`: the base cost).
fn get_cost_replace_effect_to_apply(d: &mut Duel, base_chk: bool, extracon: &LuaFn, a: &EffArgs) -> Option<EffectId> {
    let tp = a.tp;
    let effs: Vec<EffectId> = duel::get_player_effect(d, tp, EFFECT_COST_REPLACE).into_iter().filter(|&e| check_cost_replace_effect(d, e, extracon, a)).collect();
    if effs.is_empty() {
        return None;
    }
    if effs.len() == 1 {
        let h = effect::get_handler(d, effs[0]).unwrap();
        if base_chk && !duel::select_effect_yes_no(d, tp, h, None) {
            return None;
        }
        return Some(effs[0]);
    }
    let mut handlers = CardSet::new();
    let mut per_card: Vec<(CardId, Vec<EffectId>)> = Vec::new();
    for &e in &effs {
        let ec = effect::get_handler(d, e).unwrap();
        handlers.insert(ec);
        match per_card.iter_mut().find(|x| x.0 == ec) {
            Some(x) => x.1.push(e),
            None => per_card.push((ec, vec![e])),
        }
    }
    if handlers.len() == 1 {
        let h = crate::lua::group::get_first(d, &handlers).unwrap();
        if base_chk && !duel::select_effect_yes_no(d, tp, h, None) {
            return None;
        }
        return Some(select_cost_replace_effect(d, tp, &effs));
    }
    loop {
        if base_chk && !duel::select_yes_no(d, tp, 98) {
            return None;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RESOLVEEFFECT as u64);
        if let Some(g) = crate::lua::group::select(d, &handlers, tp, 1, 1, base_chk, crate::lua::NO_EX) {
            if let Some(first) = crate::lua::group::get_first(d, &g) {
                let list = per_card.iter().find(|x| x.0 == first).unwrap().1.clone();
                return Some(select_cost_replace_effect(d, tp, &list));
            }
        }
    }
}
/// `Cost.Replaceable(base, extracon)` (`aux.CostWithReplace`)
pub fn replaceable(base: LuaFn, extracon: Option<LuaFn>) -> LuaFn {
    let extracon = extracon.unwrap_or_else(aux_true);
    LuaFn::new(move |d, args| {
        let a = EffArgs::with_chk(args);
        let base_chk = base.call(d, &args_with_chk(&a, 0)).truthy();
        if a.chk == Some(0) {
            if base_chk {
                return Value::Bool(true);
            }
            for eff in duel::get_player_effect(d, a.tp, EFFECT_COST_REPLACE) {
                if check_cost_replace_effect(d, eff, &extracon, &a) {
                    return Value::Bool(true);
                }
            }
            return Value::Bool(false);
        }
        let Some(eff) = get_cost_replace_effect_to_apply(d, base_chk, &extracon, &a) else {
            return base.call(d, &args_with_chk(&a, 1));
        };
        let h = effect::get_handler(d, eff).unwrap();
        let code = card::get_original_code(d, h);
        duel::hint(d, HINT_CARD as u8, 0, code as u64);
        let Some(op) = effect::get_operation(d, eff) else {
            effect::use_count_limit(d, eff, a.tp, 1, false);
            return Value::Nil;
        };
        let mut full = vec![Value::Effect(eff), Value::Fn(extracon.clone()), Value::Effect(a.e), Value::Int(a.tp as i64)];
        full.extend(args_with_chk(&a, 1).into_iter().skip(2));
        let res = op.call(d, &full);
        effect::use_count_limit(d, eff, a.tp, 1, false);
        res
    })
}
