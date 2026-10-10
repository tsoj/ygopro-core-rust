//! Roaring Ocean Snake
use super::prelude::*;

pub const ID: u32 = 19066538;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(98049915), fusion::Mat::Code(2118022)]);
}
