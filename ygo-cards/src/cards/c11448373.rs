//! Grave Protector
use super::prelude::*;

pub const ID: u32 = 11448373;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // todeck
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(rmtarget)));
    effect::set_value(d, e1, LOCATION_DECKSHF as i64);
    card::register_effect(d, c, e1, false);
}
fn rmtarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_reason(d, c, REASON_BATTLE)
}
