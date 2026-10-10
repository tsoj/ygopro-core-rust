//! Humanoid Worm Drake
use super::prelude::*;

pub const ID: u32 = 5600127;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(73216412), fusion::Mat::Code(46821314)]);
}
