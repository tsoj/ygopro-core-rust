//! Great Mammoth of Goldfine
use super::prelude::*;

pub const ID: u32 = 54622031;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,29491031,66672569)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(29491031), fusion::Mat::Code(66672569)]);
}
