//! Hunter Owl
//! ハンター・アウル
use super::prelude::*;

pub const ID: u32 = 51962254;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be attacked
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(lua_fn(atcon)));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.upval)
    effect::set_value_fn(d, e2, effect_card_value(upval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WIND)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND)
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e(0));
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.upval(e,c)`
fn upval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.upfilter,c:GetControler(),LOCATION_MZONE,0,nil)*500
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&upfilter), p, LOCATION_MZONE, 0, NO_EX) as i64 * 500
}
/// `s.upfilter(c)`
fn upfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WIND)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND)
}
