//! Respect Play
use super::prelude::*;

pub const ID: u32 = 8951260;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    //
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e2, EFFECT_PUBLIC);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetCondition(s.con1)
    effect::set_condition(d, e2, Some(condition(con1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e3, EFFECT_PUBLIC);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,LOCATION_HAND)
    effect::set_target_range(d, e3, 0, LOCATION_HAND);
    // e3:SetCondition(s.con2)
    effect::set_condition(d, e3, Some(condition(con2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn con1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer()
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e)
}
fn con2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()~=e:GetHandlerPlayer()
    duel::get_turn_player(d) != effect::get_handler_player(d, a.e)
}
