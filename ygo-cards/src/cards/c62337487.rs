//! Fortress Whale
use super::prelude::*;

pub const ID: u32 = 62337487;

// s.listed_names={77454922}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[77454922], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
