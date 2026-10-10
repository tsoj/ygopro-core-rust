//! Doriado's Blessing
use super::prelude::*;

pub const ID: u32 = 23965037;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcGreaterCode(c,3,nil,99414168)
    ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(3)), None, &[99414168]);
}
