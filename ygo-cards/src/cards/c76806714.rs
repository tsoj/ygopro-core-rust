//! Turtle Oath
use super::prelude::*;

pub const ID: u32 = 76806714;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,91782219)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[91782219]);
}
