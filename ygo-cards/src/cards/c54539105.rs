//! War-Lion Ritual
use super::prelude::*;

pub const ID: u32 = 54539105;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,7,nil,33951077)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[33951077]);
}
