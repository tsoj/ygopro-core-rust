//! Traffic Ghost
use super::prelude::*;

pub const ID: u32 = 41248270;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, None, 3, Some(3), None, None, None);
}
