//! Emblem of the Awakening
use super::prelude::*;

pub const ID: u32 = 9845733;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,4,nil,10789972)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(4)), None, &[10789972]);
}
