//! Zera Ritual
use super::prelude::*;

pub const ID: u32 = 81756897;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,69123138)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[69123138]);
}
