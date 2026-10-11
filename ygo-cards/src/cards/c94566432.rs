//! Kaiser Dragon
use super::prelude::*;

pub const ID: u32 = 94566432;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,87796900,20315854)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(87796900), fusion::Mat::Code(20315854)]);
}
