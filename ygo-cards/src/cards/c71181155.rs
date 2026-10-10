//! Phantom Beast Cross-Wing
use super::prelude::*;

pub const ID: u32 = 71181155;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    effect::set_value(d, e1, 300);
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // aux.TargetBoolFunction(Card.IsSetCard,SET_PHANTOM_BEAST)
    card::is_set_card(d, c, &[SET_PHANTOM_BEAST as u16])
}
