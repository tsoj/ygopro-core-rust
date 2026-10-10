//! Gallugallud the Endless Enigma
use super::prelude::*;

pub const ID: u32 = 32872239;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, None, None);
}
