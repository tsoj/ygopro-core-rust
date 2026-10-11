//! Final Ritual of the Ancients
use super::prelude::*;

pub const ID: u32 = 60369732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,62420419)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[62420419]);
}
