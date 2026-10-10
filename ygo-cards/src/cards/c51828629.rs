//! Giltia the D. Knight
use super::prelude::*;

pub const ID: u32 = 51828629;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(89272878), fusion::Mat::Code(10071456)]);
}
