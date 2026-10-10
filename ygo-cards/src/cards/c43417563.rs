//! Commencement Dance
use super::prelude::*;

pub const ID: u32 = 43417563;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[4849037]);
}
