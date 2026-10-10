//! Phalanx Pike
use super::prelude::*;

pub const ID: u32 = 42709949;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsCode,0,LOCATION_GRAVE,LOCATION_GRAVE,nil,c:GetCode())*900
    let codes = card::get_code(d, c);
    duel::get_matching_group_count(d, Some(&|d, cc| card::is_code(d, cc, &codes)), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64 * 900
}
