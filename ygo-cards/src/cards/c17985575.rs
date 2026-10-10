//! Lord of D.
use super::prelude::*;

pub const ID: u32 = 17985575;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(etarget)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
}
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON)
    card::is_race(d, c, RACE_DRAGON)
}
