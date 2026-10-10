//! Berserker of the Tenyi
use super::prelude::*;

pub const ID: u32 = 5402805;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    card::enable_revive_limit(d, c);
}
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
