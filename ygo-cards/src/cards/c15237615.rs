//! Empress Judge
use super::prelude::*;

pub const ID: u32 = 15237615;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(5901497), fusion::Mat::Code(64501875)]);
}
