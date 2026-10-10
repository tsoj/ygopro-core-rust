//! Amphibious Bugroth
use super::prelude::*;

pub const ID: u32 = 40173854;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,58314394,85448931)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(58314394), fusion::Mat::Code(85448931)]);
}
