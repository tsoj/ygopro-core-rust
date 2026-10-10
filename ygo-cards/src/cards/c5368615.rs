//! Steam Gyroid
use super::prelude::*;

pub const ID: u32 = 5368615;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_ROID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,18325492,44729197)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(18325492), fusion::Mat::Code(44729197)]);
}
