//! Hungry Burger
use super::prelude::*;

pub const ID: u32 = 30243636;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={80811661}
    listed_names: &[80811661],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
