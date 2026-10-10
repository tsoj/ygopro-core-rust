//! Book of Secret Arts
use super::prelude::*;

pub const ID: u32 = 91595718;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_SPELLCASTER))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_SPELLCASTER))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
    // def up
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    effect::set_value(d, e3, 300);
    card::register_effect(d, c, e3, false);
}
