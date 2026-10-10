//! Black Illusion Ritual
use super::prelude::*;

pub const ID: u32 = 41426869;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(1)), None, &[64631466]);
}
