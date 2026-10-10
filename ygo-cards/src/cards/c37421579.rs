//! Charubin the Fire Knight
use super::prelude::*;

pub const ID: u32 = 37421579;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(36121917), fusion::Mat::Code(96851799)]);
}
