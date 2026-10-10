//! Paralyzing Potion
use super::prelude::*;

pub const ID: u32 = 50152549;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atklimit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsRace(RACE_MACHINE)
    !card::is_race(d, c, RACE_MACHINE)
}
