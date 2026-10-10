//! Beastly Mirror Ritual
use super::prelude::*;

pub const ID: u32 = 81933259;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,6,nil,31890399)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[31890399]);
}
