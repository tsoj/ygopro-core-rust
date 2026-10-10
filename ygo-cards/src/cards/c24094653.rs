//! Polymerization
use super::prelude::*;

pub const ID: u32 = 24094653;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    fusion::register_summon_eff(d, c, fusion::SummonEff::default());
}
