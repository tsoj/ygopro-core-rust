//! The Legendary Fisherman
use super::prelude::*;

pub const ID: u32 = 3643300;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // immune spell
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(econ)));
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value(d, e2, 1);
    effect::set_condition(d, e2, Some(condition(econ)));
    card::register_effect(d, c, e2, false);
}
fn econ(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_UMI)
    duel::is_environment(d, CARD_UMI, None, None)
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellEffect()
    effect::is_spell_effect(d, a.e(1))
}
