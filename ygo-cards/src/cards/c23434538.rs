//! Maxx "C"
use super::prelude::*;

pub const ID: u32 = 23434538;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(drcon1)));
    effect::set_operation(d, e1, Some(operation(drop1)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    //sp_summon effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(regcon)));
    effect::set_operation(d, e2, Some(operation(regop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_condition(d, e3, Some(condition(drcon2)));
    effect::set_operation(d, e3, Some(operation(drop2)));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e3, a.tp);
}
fn filter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    card::get_summon_player(d, c) == sp
}
fn drcon1(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - a.tp), 1, NO_EX)
        && (!effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIONS) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_CONTINUOUS))
}
fn drop1(d: &mut Duel, a: &EffArgs) {
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, 1 - a.tp), 1, NO_EX)
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIONS)
        && !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_CONTINUOUS)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
}
fn drcon2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, a.tp, ID) > 0
}
fn drop2(d: &mut Duel, a: &EffArgs) {
    let n = duel::get_flag_effect(d, a.tp, ID);
    duel::reset_flag_effect(d, a.tp, ID);
    duel::draw(d, a.tp, n as u32, REASON_EFFECT);
}
