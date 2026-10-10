//! Warrior of Tradition
use super::prelude::*;

pub const ID: u32 = 56413937;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,38942059,16899564)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(38942059), fusion::Mat::Code(16899564)]);
}
