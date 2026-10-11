//! Qardan the Clear-Sighted
use super::prelude::*;

pub const ID: u32 = 86605515;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 Continuous Trap from hand or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(sttg)));
    effect::set_operation(d, e1, Some(operation(stop)));
    card::register_effect(d, c, e1, false);
    // Make face-up Traps you control be indestructible
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
fn stfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsContinuousTrap() and c:IsSSetable()
    card::is_continuous_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn sttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&stfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
fn stop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let f = aux::necro_valley_filter(stfilter);
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if card::is_s_setable(d, tc, false, None) && duel::s_set(d, tp, CardSet::from_card(tc), None, true) > 0 {
            // Can be activated this turn
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, aux::stringid(ID, 2));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 2, 0);
    let c = effect::get_handler(d, a.e).nil();
    // Your face-up Traps cannot be destroyed by the opponent's effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_target_range(d, e1, LOCATION_ONFIELD, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_faceup(d, c) && card::is_trap(d, c))));
    effect::set_value_fn(d, e1, aux::indoval());
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e1, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, Some(2));
}
