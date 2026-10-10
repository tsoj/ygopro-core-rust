//! Reversal of Fate
use super::prelude::*;

pub const ID: u32 = 36690018;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARCANA_FORCE as u16]) && card::get_flag_effect(d, c, CARD_REVERSAL_OF_FATE) > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && filter(d, tc) {
        let val = aux::arcana::get_coin_result(d, tc);
        if val == Some(COIN_HEADS as i64) {
            aux::arcana::set_coin_result(d, tc, COIN_TAILS as i64);
        } else if val == Some(COIN_TAILS as i64) {
            aux::arcana::set_coin_result(d, tc, COIN_HEADS as i64);
        }
    }
}
