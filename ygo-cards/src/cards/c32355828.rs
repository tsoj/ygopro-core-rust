//! Skelgon
use super::prelude::*;

pub const ID: u32 = 32355828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,29491031,87564352)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(29491031), fusion::Mat::Code(87564352)]);
}
