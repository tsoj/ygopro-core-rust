//! Earth Chant
use super::prelude::*;

pub const ID: u32 = 59820352;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcEqual(c,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_EARTH))
    ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_attribute(d, c, ATTRIBUTE_EARTH))),
            ..Default::default()
        },
    );
}
