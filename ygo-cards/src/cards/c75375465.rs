//! Pandemonium Watchbear
use super::prelude::*;

pub const ID: u32 = 75375465;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94585852], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    effect::set_value_fn(d, e1, lua_fn(indval));
    card::register_effect(d, c, e1, false);
}
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[94585852])
}
/// `function(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_controler(d, h) != a.p(2)
}
