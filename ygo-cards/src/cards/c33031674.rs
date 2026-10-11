//! Incandescent Ordeal
//! 灼熱の試練
use super::prelude::*;

pub const ID: u32 = 33031674;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,7,nil,60258960)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[60258960]);
}
