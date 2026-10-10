//! Gaia Knight, the Force of Earth
use super::prelude::*;

pub const ID: u32 = 97204936;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
}
