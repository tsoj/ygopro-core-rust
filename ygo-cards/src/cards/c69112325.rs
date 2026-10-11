//! Spiritual Forest
use super::prelude::*;

pub const ID: u32 = 69112325;

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
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.indtg)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| indtg(d, c))));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetValue(s.valcon)
    effect::set_value_fn(d, e2, lua_fn(valcon));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT|RACE_BEAST|RACE_BEASTWARRIOR)
    card::is_race(d, c, RACE_PLANT | RACE_BEAST | RACE_BEASTWARRIOR)
}
/// `s.valcon(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
