//! Flame Ghost
use super::prelude::*;

pub const ID: u32 = 58528964;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_SKULL_SERVANT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion summon procedure
    // Fusion.AddProcMix(c,true,true,CARD_SKULL_SERVANT,40826495)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_SKULL_SERVANT), fusion::Mat::Code(40826495)]);
}
