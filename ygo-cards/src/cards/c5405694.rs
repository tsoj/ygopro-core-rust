//! Black Luster Soldier
use super::prelude::*;

pub const ID: u32 = 5405694;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[55761792], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
