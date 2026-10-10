//! Level Limit - Area B
use super::prelude::*;

pub const ID: u32 = 3136426;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Pos Change
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SET_POSITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_value(d, e2, POS_FACEUP_DEFENSE as i64);
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetLevel()>3 and c:IsFaceup()
    card::get_level(d, c) > 3 && card::is_faceup(d, c)
}
