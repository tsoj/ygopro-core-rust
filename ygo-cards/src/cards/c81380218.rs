//! Chorus of Sanctuary
use super::prelude::*;

pub const ID: u32 = 81380218;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Def up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // aux.TargetBoolFunction(Card.IsDefensePos)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_defense_pos(d, c))));
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
}
