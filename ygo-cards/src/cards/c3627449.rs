//! Skull Guardian
use super::prelude::*;

pub const ID: u32 = 3627449;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[43694075], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
