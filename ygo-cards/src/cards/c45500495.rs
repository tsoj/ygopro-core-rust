//! Queen of Thorns
use super::prelude::*;

pub const ID: u32 = 45500495;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_PLANT),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_PLANT, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e1, EFFECT_SUMMON_COST);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    effect::set_target_range(d, e1, LOCATION_HAND, LOCATION_HAND);
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(effect_card_fn(sumtg)));
    // e1:SetCost(s.ccost)
    effect::set_cost(d, e1, Some(lua_fn(ccost)));
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e2, EFFECT_SPSUMMON_COST);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.sumtg(e,c) return c:GetRace()~=RACE_PLANT end
fn sumtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::get_race(d, c) != RACE_PLANT
}
// function s.ccost(e,c,tp) return Duel.CheckLPCost(tp,1000) end
fn ccost(d: &mut Duel, a: Args) -> bool {
    duel::check_lp_cost(d, a.p(2), 1000)
}
// function s.acop(e,tp,eg,ep,ev,re,r,rp) Duel.PayLPCost(tp,1000) end
fn acop(d: &mut Duel, a: &EffArgs) {
    duel::pay_lp_cost(d, a.tp, 1000);
}
