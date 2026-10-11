//! Zera the Mant
use super::prelude::*;

pub const ID: u32 = 69123138;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[81756897], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
