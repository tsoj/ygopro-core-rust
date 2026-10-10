//! Kaminari Attack
use super::prelude::*;

pub const ID: u32 = 9653271;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,86088138,21817254)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(86088138), fusion::Mat::Code(21817254)]);
}
