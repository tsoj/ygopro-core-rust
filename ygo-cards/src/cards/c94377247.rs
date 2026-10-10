//! Curse of the Masked Beast
use super::prelude::*;

pub const ID: u32 = 94377247;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,49064413)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[49064413]);
}
