//! Fortress Whale's Oath
use super::prelude::*;

pub const ID: u32 = 77454922;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,7,nil,62337487)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[62337487]);
}
