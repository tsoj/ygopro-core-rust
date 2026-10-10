//! Mystical Sand
use super::prelude::*;

pub const ID: u32 = 32751480;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,13039848,93221206)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(13039848), fusion::Mat::Code(93221206)]);
}
