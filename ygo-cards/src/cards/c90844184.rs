//! Garma Sword
use super::prelude::*;

pub const ID: u32 = 90844184;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[78577570],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
