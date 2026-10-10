//! Musician King
use super::prelude::*;

pub const ID: u32 = 56907389;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,78010363,17358176)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(78010363), fusion::Mat::Code(17358176)]);
}
