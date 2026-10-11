//! Lion Alligator
use super::prelude::*;

pub const ID: u32 = 4611269;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.condition(e)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_REPTILE),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    let reptile = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_REPTILE as u64));
    duel::is_existing_matching_card(d, Some(&reptile), p, LOCATION_MZONE, 0, 1, h.into())
}
// s.target(e,c)
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE)
    card::is_race(d, c, RACE_REPTILE as u64)
}
