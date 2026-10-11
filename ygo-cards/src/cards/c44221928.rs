//! Breath of Acclamation
use super::prelude::*;

pub const ID: u32 = 44221928;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual.AddProcEqual(c,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_WIND))
    ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_attribute(d, x, ATTRIBUTE_WIND))),
            ..Default::default()
        },
    );
}
