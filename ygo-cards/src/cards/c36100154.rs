//! Amazoness Fighting Spirit
use super::prelude::*;

pub const ID: u32 = 36100154;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // "Amazoness" monsters gain 1000 ATK during Damage Calculation
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // s[0]=false
    set_s_var(d, ID, 0, Value::Bool(false));
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    duel::get_attacker(d) == Some(c) && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let dt = duel::get_attack_target(d);
    if s_var(d, ID, 0).truthy() || card::get_attack(d, c) < card::get_attack(d, dt.nil()) {
        set_s_var(d, ID, 0, Value::Bool(true));
        1000
    } else {
        0
    }
}
