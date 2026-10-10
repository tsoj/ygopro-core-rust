//! End of the World
use super::prelude::*;

pub const ID: u32 = 8198712;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcEqualCode(c,nil,nil,72426662,46427957)
    ritual::add_proc_equal_code(d, c, None, None, &[72426662, 46427957]);
}
