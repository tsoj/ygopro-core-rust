//! Kwagar Hercules
use super::prelude::*;

pub const ID: u32 = 95144193;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,60802233,52584282)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(60802233), fusion::Mat::Code(52584282)]);
}
