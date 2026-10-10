//! Wall Shadow
use super::prelude::*;

pub const ID: u32 = 63162310;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
