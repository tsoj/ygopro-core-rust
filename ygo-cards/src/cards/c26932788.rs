//! Javelin Beetle
use super::prelude::*;

pub const ID: u32 = 26932788;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41182875], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
