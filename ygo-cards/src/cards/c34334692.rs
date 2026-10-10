//! Ritual Raven
use super::prelude::*;

pub const ID: u32 = 34334692;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ritual level
    // Ritual.AddWholeLevelTribute(c,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK))
    ritual::add_whole_level_tribute(d, c, std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId| card::is_attribute(d, c, ATTRIBUTE_DARK)));
}
