//! Flame Swordsman
use super::prelude::*;

pub const ID: u32 = 45231177;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(34460851), fusion::Mat::Code(44287299)]);
}
