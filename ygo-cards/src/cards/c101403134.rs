//! Diabellze the Mistress of the White Crow Forest (alternative artwork): `Duel.LoadCardScriptAlias(101403034)`, i.e. the script of 101403034 (its
//! table, so its state is shared with the original).
use super::prelude::*;

pub const ID: u32 = 101403134;

pub const INFO: ScriptInfo = super::c101403034::INFO;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    super::c101403034::initial_effect(d, c);
}
