//! Shinato's Ark
use super::prelude::*;

pub const ID: u32 = 60365591;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,8,nil,86327225)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(8)), None, &[86327225]);
}
