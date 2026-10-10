//! Gaia the Dragon Champion
use super::prelude::*;

pub const ID: u32 = 66889139;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_GAIA_THE_FIERCE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(6368038), fusion::Mat::Code(28279543)]);
}
