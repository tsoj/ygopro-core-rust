//! Rare Metal Dragon
use super::prelude::*;

pub const ID: u32 = 25236056;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
}
