//! Security Block
use super::prelude::*;

pub const ID: u32 = 91665064;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted cyberse monster cannot be destroyed by battle, also neither player take battle damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    card::register_effect(d, c, e1, false);
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE as u64)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}

fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // Neither player take battle damage this turn
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);

    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) {
            // Cannot be destroyed by battle
            let h = effect::get_handler(d, e).nil();
            let e2 = effect::create_effect(d, h);
            effect::set_description(d, e2, 3000);
            effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            effect::set_value(d, e2, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
}
