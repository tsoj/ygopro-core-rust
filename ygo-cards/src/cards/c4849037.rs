//! Performance of Sword
use super::prelude::*;

pub const ID: u32 = 4849037;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[43417563], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
