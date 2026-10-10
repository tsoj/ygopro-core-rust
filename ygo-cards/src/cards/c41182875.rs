//! Javelin Beetle Pact
use super::prelude::*;

pub const ID: u32 = 41182875;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[26932788]);
}
