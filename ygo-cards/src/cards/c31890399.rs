//! Fiend's Mirror
use super::prelude::*;

pub const ID: u32 = 31890399;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[81933259],
    ..ScriptInfo::DEFAULT
};
