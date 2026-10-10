//! Allvain the Essence of Vanity
use super::prelude::*;

pub const ID: u32 = 3544583;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), st, Some(tp)))), 2)]);
}
