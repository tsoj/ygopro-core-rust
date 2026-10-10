//! Man-eating Black Shark
use super::prelude::*;

pub const ID: u32 = 80727036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,71746462,65623423,17968114)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(71746462), fusion::Mat::Code(65623423), fusion::Mat::Code(17968114)],
    );
}
