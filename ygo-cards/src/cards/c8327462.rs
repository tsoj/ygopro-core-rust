//! Skullbird
use super::prelude::*;

pub const ID: u32 = 8327462;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,3170832,732302)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(3170832), fusion::Mat::Code(732302)]);
}
