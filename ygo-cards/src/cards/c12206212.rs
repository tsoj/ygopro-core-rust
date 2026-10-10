//! Harpie Lady Sisters
use super::prelude::*;

pub const ID: u32 = 12206212;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
