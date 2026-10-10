//! Droll & Lock Bird
use super::prelude::*;

pub const ID: u32 = 94145021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_TO_HAND);
        effect::set_condition(d, ge1, Some(condition(regcon)));
        effect::set_operation(d, ge1, Some(operation(regop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    if duel::is_phase(d, PHASE_DRAW) || duel::is_phase(d, PHASE_DAMAGE) {
        return false;
    }
    let mut v: usize = 0;
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| cfilter(d, c, 0), 1, NO_EX) {
        v += 1;
    }
    if group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1), 1, NO_EX) {
        v += 2;
    }
    if v == 0 {
        return false;
    }
    let labels = [0, 1, PLAYER_ALL as i64];
    effect::set_label(d, a.e, &[labels[v - 1]]);
    true
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    duel::raise_event(d, a.eg(), EVENT_CUSTOM + ID, a.re, a.r, a.rp, a.ep, label as u32);
}
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ev == (1 - a.tp) as u32 || a.ev == PLAYER_ALL
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_location(d, c, LOCATION_HAND) && card::is_able_to_grave_as_cost(d, c);
    }
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
    effect::set_target_range(d, e1, LOCATION_DECK, LOCATION_DECK);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 0);
    duel::register_effect(d, e1, a.tp);

    let e2 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_DRAW);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 0);
    effect::set_target_range(d, e2, 1, 1);
    duel::register_effect(d, e2, a.tp);

    let e3 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 0);
    effect::set_target_range(d, e3, 1, 1);
    duel::register_effect(d, e3, a.tp);
}
