//! Soul Hunter
use super::prelude::*;

pub const ID: u32 = 72869010;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(99510761), fusion::Mat::Code(28450915)]);
}
