//! Deepsea Shark
use super::prelude::*;

pub const ID: u32 = 28593363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,81386177,69572024)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(81386177), fusion::Mat::Code(69572024)]);
}
