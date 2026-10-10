//! Cyber Saurus
use super::prelude::*;

pub const ID: u32 = 89112729;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(70138455), fusion::Mat::Code(94119974)]);
}
