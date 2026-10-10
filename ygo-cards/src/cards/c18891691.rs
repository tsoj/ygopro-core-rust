//! Perfect Machine King
use super::prelude::*;

pub const ID: u32 = 18891691;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.val(e,c)
fn val(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_MACHINE),c:GetControler(),LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())*500
    let tp = card::get_controler(d, c);
    let h = effect::get_handler(d, e).nil();
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_MACHINE)), tp, LOCATION_MZONE, LOCATION_MZONE, h.into()) as i64 * 500
}
