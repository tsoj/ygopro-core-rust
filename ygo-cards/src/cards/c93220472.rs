//! Cave Dragon
use super::prelude::*;

pub const ID: u32 = 93220472;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // sumlimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(lua_fn(sumcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atklimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(lua_fn(atkcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.sumcon(e)
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)>0
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) > 0
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    !duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_DRAGON as u64))), p, LOCATION_MZONE, 0, 1, h.into())
}
