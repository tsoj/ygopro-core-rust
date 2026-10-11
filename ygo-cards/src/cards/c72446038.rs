//! Synthesis Spell
use super::prelude::*;

pub const ID: u32 = 72446038;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,6,nil,84385264)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(6)), None, &[84385264]);
}
