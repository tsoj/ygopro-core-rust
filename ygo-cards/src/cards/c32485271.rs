//! Rose Spectre of Dunn
use super::prelude::*;

pub const ID: u32 = 32485271;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,41392891,29802344)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(41392891), fusion::Mat::Code(29802344)]);
}
