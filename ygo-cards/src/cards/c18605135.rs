//! Tornado Wall
use super::prelude::*;

pub const ID: u32 = 18605135;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(umicheck)));
    card::register_effect(d, c, e1, false);
    // avoid battle damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_condition(d, e2, Some(condition(umicheck)));
    card::register_effect(d, c, e2, false);
    // self destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e3, Some(condition(not_umicheck)));
    card::register_effect(d, c, e3, false);
}
fn umicheck(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_UMI)
    duel::is_environment(d, CARD_UMI, None, None)
}
fn not_umicheck(d: &mut Duel, a: &EffArgs) -> bool {
    // aux.NOT(s.umicheck)
    !umicheck(d, a)
}
