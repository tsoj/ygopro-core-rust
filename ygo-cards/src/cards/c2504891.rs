//! Skull Knight
use super::prelude::*;

pub const ID: u32 = 2504891;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(28725004), fusion::Mat::Code(42431843)]);
}
