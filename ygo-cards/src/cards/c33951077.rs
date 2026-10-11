//! Super War-Lion
use super::prelude::*;

pub const ID: u32 = 33951077;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[54539105], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
