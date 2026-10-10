//! Flower Wolf
use super::prelude::*;

pub const ID: u32 = 95952802;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,90357090,43500484)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(90357090), fusion::Mat::Code(43500484)]);
}
