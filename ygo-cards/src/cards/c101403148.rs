//! The King of Beasts that Rises from the Valley of Flames (alternative artwork): `Duel.LoadCardScriptAlias(101403048)`, i.e. the script of 101403048 (its
//! table, so its state is shared with the original).
use super::prelude::*;

pub const ID: u32 = 101403148;

pub const INFO: ScriptInfo = super::c101403048::INFO;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    super::c101403048::initial_effect(d, c);
}
