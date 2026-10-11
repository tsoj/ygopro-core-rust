//! Wattcancel
use super::prelude::*;

pub const ID: u32 = 56993276;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WATT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON);
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_current_chain(d, true) == 0
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_WATT as u16]) && card::is_monster(d, c) && card::is_discardable(d, c, None, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, EFFECT_DISCARD_COST_CHANGE).is_some() {
        return true;
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    duel::negate_summon(d, g.clone());
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
