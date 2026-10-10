//! Gagagigo the Risen
use super::prelude::*;

pub const ID: u32 = 9053187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
}
