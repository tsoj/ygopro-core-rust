//! Chthonian Alliance
use super::prelude::*;

pub const ID: u32 = 46910446;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(value));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,code)`
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(code)
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,c,c:GetCode())*800
    // (c:GetCode() is the last argument, so every code it returns is passed; the filter takes the first)
    let code = card::get_code(d, c).first().copied().nil();
    duel::get_matching_group_count(d, Some(&|d, x| filter(d, x, code)), 0, LOCATION_MZONE, LOCATION_MZONE, c.into()) as i64 * 800
}
