//! The `aux.*` of `proc_workaround.lua`: release checks for Tribute costs and Tribute summons.

use crate::constants::*;
use crate::lua::{card, duel, effect, group, Filter, NO_EX};
use crate::prelude::*;

/// The `check` of `Duel.CheckReleaseGroupCost`/`SelectReleaseGroupCost`:
/// `function(sg, tp, exg, ...)` returning (the group is valid, stop extending it); the `...`
/// arguments go into the closure.
pub type ReleaseCheck<'a> = &'a dyn Fn(&mut Duel, &CardSet, u8, &CardSet) -> (bool, bool);

/// `aux.ReleaseNonSumCheck(c, tp, e)`: an opponent's monster that can be Tributed for `e`
pub fn release_non_sum_check(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    if card::is_controler(d, c, tp) {
        return false;
    }
    let mut chk = false;
    for eff in card::get_card_effect(d, c, EFFECT_EXTRA_RELEASE_NONSUM) {
        chk = match effect::get_value(d, eff) {
            Value::Fn(f) => f.call(d, &[Value::Effect(eff), Value::effect(e), Value::Int(REASON_COST as i64), Value::Int(tp as i64)]).truthy(),
            v => v.int() != 0,
        };
        if chk {
            return true;
        }
    }
    chk
}
/// `aux.ZoneCheckFunc(c, tp, zone)`: the free zones `c` would have after Tributing `sg`
pub fn zone_check_func(d: &mut Duel, c: CardId, tp: u8, zone: u32, sg: &CardSet) -> i32 {
    if card::is_location(d, c, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(c), None).0
    } else {
        duel::get_m_zone_count(d, tp, sg.into(), None, None, Some(zone)).0
    }
}
/// `aux.ReleaseCheckSingleUse(sg, tp, exg)`: at most one of the opponent's monsters
pub fn release_check_single_use(_d: &mut Duel, sg: &CardSet, _tp: u8, exg: &CardSet) -> (bool, bool) {
    let ct = group::intersection(sg, exg).len();
    (ct <= 1, ct > 1)
}
/// `aux.ReleaseCheckMMZ(sg, tp)`: a Main Monster Zone is free after Tributing
pub fn release_check_mmz(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    (duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0, false)
}
/// `aux.ReleaseCheckTarget(sg, tp, exg, dg)`: a card of `dg` is left to target
pub fn release_check_target(d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet, dg: &CardSet) -> (bool, bool) {
    (group::is_exists(d, dg, &|_, _| true, 1, sg.into()), false)
}
struct RelCtx<'a> {
    mg: &'a CardSet,
    mustg: &'a CardSet,
    minc: usize,
    maxc: usize,
    specialchk: &'a dyn Fn(&mut Duel, &CardSet) -> (bool, bool),
}
/// `aux.RelCheckRecursive(c, tp, sg, mg, exg, mustg, ct, minc, maxc, specialchk)`
fn rel_check_recursive(d: &mut Duel, x: &RelCtx, c: CardId, sg: &mut CardSet, ct: usize) -> bool {
    sg.insert(c);
    let ct = ct + 1;
    let (mut res, stop) = rel_check_goal(d, x, sg, ct);
    if !res && !stop {
        res = ct < x.maxc && {
            let mut found = false;
            for c2 in group::iter(d, x.mg) {
                if sg.contains(c2) {
                    continue;
                }
                if crate::lua::protected(d, false, |d| rel_check_recursive(d, x, c2, sg, ct)) {
                    found = true;
                    break;
                }
            }
            found
        };
    }
    sg.remove(c);
    res
}
/// `aux.RelCheckGoal(tp, sg, exg, mustg, ct, minc, maxc, specialchk)`
fn rel_check_goal(d: &mut Duel, x: &RelCtx, sg: &CardSet, ct: usize) -> (bool, bool) {
    if ct < x.minc || ct > x.maxc {
        return (false, ct > x.maxc);
    }
    let (res, stop) = (x.specialchk)(d, sg);
    (res && group::includes(sg, x.mustg), stop)
}
/// `aux.ReleaseCostFilter(c, tp)`
pub fn release_cost_filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let eff = card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE_NONSUM, None).first().copied();
    !(card::is_controler(d, c, 1 - tp) && eff.is_some_and(|eff| effect::check_count_limit(d, eff, tp))) && card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty()
}
/// `aux.MakeSpecialCheck(check, tp, exg, ...)`
fn special_check(d: &mut Duel, check: Option<ReleaseCheck>, tp: u8, exg: &CardSet, sg: &CardSet) -> (bool, bool) {
    match check {
        None => release_check_single_use(d, sg, tp, exg),
        Some(check) => {
            let (res, stop) = check(d, sg, tp, exg);
            let (res2, stop2) = release_check_single_use(d, sg, tp, exg);
            (res && res2, stop || stop2)
        }
    }
}
/// The groups both release cost functions start with: (mg, exg, mustg).
fn release_cost_groups(d: &mut Duel, tp: u8, f: Option<Filter>, use_hand: bool, ex: crate::lua::Exclude) -> (CardSet, CardSet, CardSet) {
    let mut mg = duel::get_release_group(d, tp, use_hand, false, None);
    group::match_(d, &mut mg, f.unwrap_or(&|_, _| true), ex);
    let (mut g, exg) = group::split(d, &mg, &|d, c| release_cost_filter(d, c, tp), NO_EX);
    group::match_(d, &mut g, &|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty() && card::is_controler(d, c, 1 - tp), NO_EX);
    (mg, exg, g)
}
/// `Duel.CheckReleaseGroupCost(tp, f, minc, maxc, use_hand, check, ex, ...)`: whether `minc`..`maxc`
/// monsters matching `f` can be Tributed for a cost (with at most 1 of the opponent's). The old
/// form without `maxc` is `maxc = minc`.
#[allow(clippy::too_many_arguments)]
pub fn check_release_group_cost(d: &mut Duel, tp: u8, f: Option<Filter>, minc: usize, maxc: usize, use_hand: bool, check: Option<ReleaseCheck>, ex: crate::lua::Exclude) -> bool {
    let (mg, exg, mustg) = release_cost_groups(d, tp, f, use_hand, ex);
    let specialchk = |d: &mut Duel, sg: &CardSet| special_check(d, check, tp, &exg, sg);
    let x = RelCtx { mg: &mg, mustg: &mustg, minc, maxc, specialchk: &specialchk };
    if !group::includes(&mg, &mustg) {
        return false;
    }
    let mut sg = CardSet::new();
    for c in group::iter(d, &mg) {
        if crate::lua::protected(d, false, |d| rel_check_recursive(d, &x, c, &mut sg, 0)) {
            return true;
        }
    }
    false
}
/// `Duel.SelectReleaseGroupCost(tp, f, minc, maxc, use_hand, check, ex, ...)`: the monsters chosen
/// one by one (empty if none)
#[allow(clippy::too_many_arguments)]
pub fn select_release_group_cost(d: &mut Duel, tp: u8, f: Option<Filter>, minc: usize, maxc: usize, use_hand: bool, check: Option<ReleaseCheck>, ex: crate::lua::Exclude) -> CardSet {
    let (mg, exg, mustg) = release_cost_groups(d, tp, f, use_hand, ex);
    let specialchk = |d: &mut Duel, sg: &CardSet| special_check(d, check, tp, &exg, sg);
    let x = RelCtx { mg: &mg, mustg: &mustg, minc, maxc, specialchk: &specialchk };
    let mut sg = CardSet::new();
    group::add_card(&mut sg, &mustg);
    while sg.len() < maxc {
        let mut cg = CardSet::new();
        for c in group::iter(d, &mg) {
            if sg.contains(c) {
                continue;
            }
            let mut tmp = sg.clone();
            if crate::lua::protected(d, false, |d| rel_check_recursive(d, &x, c, &mut tmp, sg.len())) {
                cg.insert(c);
            }
        }
        if cg.is_empty() {
            break;
        }
        let cancel = rel_check_goal(d, &x, &sg, sg.len()).0;
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        let Some(tc) = group::select_unselect(d, &cg, Some(&sg), tp, cancel, cancel, Some(1), Some(1)) else { break };
        if mustg.is_empty() || !mustg.contains(tc) {
            if !sg.contains(tc) {
                sg.insert(tc);
            } else {
                sg.remove(tc);
            }
        }
    }
    if sg.is_empty() {
        return sg;
    }
    let both = group::intersection(&sg, &exg);
    if !both.is_empty() {
        let first = group::get_first(d, &both).nil();
        if let Some(&eff) = card::is_has_effect(d, first, EFFECT_EXTRA_RELEASE_NONSUM, None).first() {
            effect::use_count_limit(d, eff, tp, 1, false);
            let h = effect::get_handler(d, eff).nil();
            let code = card::get_code(d, h)[0];
            duel::hint(d, HINT_CARD as u8, 0, code as u64);
        }
    }
    sg
}

// `Duel.CheckReleaseGroupSummon`/`SelectReleaseGroupSummon` are not ported: no script uses them
// (and in the core both raise an error once there are enough monsters: they pass a number as the
// exclusion of `IsExists`).
