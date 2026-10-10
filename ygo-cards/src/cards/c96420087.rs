//! Contract with the Dark Master
use super::prelude::*;

pub const ID: u32 = 96420087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,97642679)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[97642679]);
}
