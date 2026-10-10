//! Gaia Saber, the Lightning Shadow
use super::prelude::*;

pub const ID: u32 = 67598234;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, None, 2, None, None, None, None);
    card::enable_revive_limit(d, c);
}
