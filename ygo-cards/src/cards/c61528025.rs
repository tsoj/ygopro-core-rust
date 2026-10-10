//! Banisher of the Light
use super::prelude::*;

pub const ID: u32 = 61528025;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_IGNORE_RANGE | EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0xff, 0xff);
    effect::set_target(d, e1, Some(effect_card_fn(rmtarget)));
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e1, false);
}
fn rmtarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    duel::is_player_can_remove(d, p, Some((c, None)))
}
