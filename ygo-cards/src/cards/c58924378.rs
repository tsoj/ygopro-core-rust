//! Wattcastle
use super::prelude::*;

pub const ID: u32 = 58924378;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WATT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // atk down
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_operation(d, e2, Some(operation(atkdn)));
    card::register_effect(d, c, e2, false);
}
fn atkdn(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    if let Some(dt) = dt {
        if card::is_set_card(d, dt, &[SET_WATT as u16]) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value(d, e1, -1000);
            card::register_effect(d, at.nil(), e1, false);
        }
    }
}
