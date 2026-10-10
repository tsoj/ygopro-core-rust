//! Polymerization (alternative artwork): `Duel.LoadCardScriptAlias(CARD_POLYMERIZATION)`, i.e. the
//! script of 24094653 (its table, so its state is shared with the original).
use super::prelude::*;

pub const ID: u32 = 27847700;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    super::c24094653::initial_effect(d, c);
}
