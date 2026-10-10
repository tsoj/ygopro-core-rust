//! Novox's Prayer
use super::prelude::*;

pub const ID: u32 = 43694075;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(7)), None, &[3627449]);
}
