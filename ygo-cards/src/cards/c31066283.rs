//! Revival of Dokurorider
use super::prelude::*;

pub const ID: u32 = 31066283;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[99721536]);
}
