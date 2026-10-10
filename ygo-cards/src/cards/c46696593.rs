//! Crimson Sunbird
use super::prelude::*;

pub const ID: u32 = 46696593;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,75582395,10202894)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(75582395), fusion::Mat::Code(10202894)]);
}
