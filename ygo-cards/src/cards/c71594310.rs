//! Gem-Knight Pearl
use super::prelude::*;

pub const ID: u32 = 71594310;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
}
