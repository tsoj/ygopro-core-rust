//! White Dragon Ritual
use super::prelude::*;

pub const ID: u32 = 9786492;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(4)), None, &[73398797]);
}
