//! Lightning Blade
use super::prelude::*;

pub const ID: u32 = 55226821;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_WARRIOR))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_WARRIOR))), None, None, None, None, None, None);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 800);
    card::register_effect(d, c, e2, false);
    // atk down
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    effect::set_target(d, e4, Some(effect_card_fn(adtg)));
    effect::set_value(d, e4, -500);
    card::register_effect(d, c, e4, false);
}
/// `s.adtg(e,c)`
fn adtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER)
    card::is_attribute(d, c, ATTRIBUTE_WATER)
}
