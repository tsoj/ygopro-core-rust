//! Vermillion Sparrow
use super::prelude::*;

pub const ID: u32 = 35752363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,62403074,71407486)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(62403074), fusion::Mat::Code(71407486)]);
}
