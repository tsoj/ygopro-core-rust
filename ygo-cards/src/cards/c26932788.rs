//! Javelin Beetle
use super::prelude::*;

pub const ID: u32 = 26932788;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
