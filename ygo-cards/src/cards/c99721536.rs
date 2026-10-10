//! Dokurorider
use super::prelude::*;

pub const ID: u32 = 99721536;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
