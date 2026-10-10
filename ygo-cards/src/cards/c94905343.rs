//! Rabid Horseman
use super::prelude::*;

pub const ID: u32 = 94905343;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(5053103), fusion::Mat::Code(68516705)]);
}
