//! Rare Fish
use super::prelude::*;

pub const ID: u32 = 80516007;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,1641882,75376965)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(1641882), fusion::Mat::Code(75376965)]);
}
