//! Twin-Headed Thunder Dragon
use super::prelude::*;

pub const ID: u32 = 54752875;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::Code(31786629), 2)]);
}
