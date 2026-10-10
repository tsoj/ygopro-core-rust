//! Barox
use super::prelude::*;

pub const ID: u32 = 6840573;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,98818516,15303296)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(98818516), fusion::Mat::Code(15303296)]);
}
