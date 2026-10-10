//! Soul Exchange
use super::prelude::*;

pub const ID: u32 = 68005187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    let unrel_sum = !card::is_has_effect(d, c, EFFECT_UNRELEASABLE_SUM, None).is_empty();
    !(unrel_sum && !card::is_has_effect(d, c, EFFECT_UNRELEASABLE_NONSUM, None).is_empty())
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_current_phase(d) != PHASE_MAIN2;
    }
    let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EXTRA_RELEASE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
