//! The Sun God of Darkness - The Winged Dragon of Ra (alternative artwork): `Duel.LoadCardScriptAlias(101403030)`, i.e. the script of 101403030 (its
//! table, so its state is shared with the original).
use super::prelude::*;

pub const ID: u32 = 101403130;

pub const INFO: ScriptInfo = super::c101403030::INFO;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    super::c101403030::initial_effect(d, c);
}
