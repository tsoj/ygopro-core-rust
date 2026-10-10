//! Marine Beast
use super::prelude::*;

pub const ID: u32 = 29929832;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,93343894,94022093)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(93343894), fusion::Mat::Code(94022093)]);
}
