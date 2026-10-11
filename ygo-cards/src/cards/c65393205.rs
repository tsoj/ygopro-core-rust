//! Chakra
use super::prelude::*;

pub const ID: u32 = 65393205;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[39399168], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
