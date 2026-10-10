//! Sanwitch
use super::prelude::*;

pub const ID: u32 = 53539634;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,78010363,26202165)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(78010363), fusion::Mat::Code(26202165)]);
}
