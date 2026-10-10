//! Aqua Dragon
use super::prelude::*;

pub const ID: u32 = 86164529;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,20315854,17968114,86100785)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(20315854), fusion::Mat::Code(17968114), fusion::Mat::Code(86100785)]);
}
