//! Black Magic Ritual
use super::prelude::*;

pub const ID: u32 = 76792184;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,30208479)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[30208479]);
}
