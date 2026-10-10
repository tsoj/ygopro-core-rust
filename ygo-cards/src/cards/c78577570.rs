//! Garma Sword Oath
use super::prelude::*;

pub const ID: u32 = 78577570;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,7,nil,90844184)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[90844184]);
}
