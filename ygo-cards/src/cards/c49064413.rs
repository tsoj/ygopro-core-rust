//! The Masked Beast
use super::prelude::*;

pub const ID: u32 = 49064413;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94377247], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
}
