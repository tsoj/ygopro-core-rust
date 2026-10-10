//! St. Joan
use super::prelude::*;

pub const ID: u32 = 21175632;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(84080938), fusion::Mat::Code(57579381)]);
}
