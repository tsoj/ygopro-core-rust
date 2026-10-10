//! Bracchio-raidus
use super::prelude::*;

pub const ID: u32 = 16507828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,94119974,38289717)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(94119974), fusion::Mat::Code(38289717)]);
}
