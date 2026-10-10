//! Crab Turtle
use super::prelude::*;

pub const ID: u32 = 91782219;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[76806714], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
}
