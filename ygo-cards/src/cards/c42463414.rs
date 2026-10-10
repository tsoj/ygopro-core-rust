//! Spined Gillman
use super::prelude::*;

pub const ID: u32 = 42463414;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    effect::set_value(d, e1, 400);
    card::register_effect(d, c, e1, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA)
    card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
}
