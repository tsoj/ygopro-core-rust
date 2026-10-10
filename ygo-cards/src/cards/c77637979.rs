//! LANphorhynchus
use super::prelude::*;

pub const ID: u32 = 77637979;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
}
