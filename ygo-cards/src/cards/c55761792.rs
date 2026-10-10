//! Black Luster Ritual
use super::prelude::*;

pub const ID: u32 = 55761792;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,5405694)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[5405694]);
}
