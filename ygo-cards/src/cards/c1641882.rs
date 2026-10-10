//! Fusionist
use super::prelude::*;

pub const ID: u32 = 1641882;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,38142739,83464209)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(38142739), fusion::Mat::Code(83464209)]);
}
