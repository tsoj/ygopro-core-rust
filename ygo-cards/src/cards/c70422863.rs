//! Hexatellarknight
use super::prelude::*;

pub const ID: u32 = 70422863;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // atk/def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
    // negate
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_code(d, e4, EVENT_BE_BATTLE_TARGET);
    effect::set_condition(d, e4, Some(condition(negcon)));
    effect::set_cost(d, e4, Some(cost_target(negcost)));
    effect::set_operation(d, e4, Some(operation(negop)));
    card::register_effect(d, c, e4, false);
}
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16])
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_overlay_count(d, c) as i64 * 200
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    card::is_faceup(d, tc)
        && card::is_type(d, tc, TYPE_XYZ)
        && card::is_set_card(d, tc, &[SET_TELLARKNIGHT as u16])
        && card::is_controler(d, tc, a.tp)
        && card::is_location(d, tc, LOCATION_MZONE)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_able_to_grave_as_cost(d, c)
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST, NO_EX);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::negate_attack(d);
}
