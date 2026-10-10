//! Magician of Black Chaos
use super::prelude::*;

pub const ID: u32 = 30208479;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[76792184], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
