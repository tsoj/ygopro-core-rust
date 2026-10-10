//! Sebek's Blessing
use super::prelude::*;

pub const ID: u32 = 22537443;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(reccon)));
    effect::set_target(d, e1, Some(cost_target(rectg)));
    effect::set_operation(d, e1, Some(operation(recop)));
    card::register_effect(d, c, e1, false);
}
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, a.ev);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, a.ev as i32);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
