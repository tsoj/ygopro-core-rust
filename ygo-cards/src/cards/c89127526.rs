//! Barrier Resonator
use super::prelude::*;

pub const ID: u32 = 89127526;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply an effect on 1 Tuner monster you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    effect::set_condition(d, e1, Some(condition(|d, _a| {
        duel::is_able_to_enter_bp(d) || (duel::is_battle_phase(d, None) && !duel::is_phase(d, PHASE_BATTLE))
    })));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_TUNER) && card::is_faceup(d, c) && !card::has_flag_effect(d, c, ID, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    let _g = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) {
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // It cannot be destroyed by battle this turn
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, aux::stringid(ID, 1));
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // You take no battle damage from attacks involving it this turn
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
        card::register_effect(d, tc, e2, false);
    }
}
