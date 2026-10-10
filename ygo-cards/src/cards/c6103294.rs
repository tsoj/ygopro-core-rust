//! Emissary of the Oasis
use super::prelude::*;

pub const ID: u32 = 6103294;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(aux.imval1)
    effect::set_value_fn(d, e1, aux::imval1());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetTarget(s.cfilter2)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| cfilter2(d, c))));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_NO_BATTLE_DAMAGE);
    // e3:SetTarget(s.cfilter2)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| cfilter2(d, c))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL) and c:IsLevelBelow(3)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 3)
}
/// `s.cfilter2(e,c)`
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsLevelBelow(3)
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 3)
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter1,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter1(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
