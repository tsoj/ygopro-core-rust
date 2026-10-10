//! Hamburger Recipe
use super::prelude::*;

pub const ID: u32 = 80811661;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,6,nil,30243636)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[30243636]);
}
