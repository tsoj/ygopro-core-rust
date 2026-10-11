//! Abyss Actors Back Stage
use super::prelude::*;

pub const ID: u32 = 59057953;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ABYSS_ACTOR}
    listed_series: &[SET_ABYSS_ACTOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_PZONE,0,2,nil,SET_ABYSS_ACTOR)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])), a.tp, LOCATION_PZONE, 0, 2, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_ABYSS_ACTOR) and not c:IsForbidden()
    card::is_type(d, c, TYPE_PENDULUM) && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && !card::is_forbidden(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=2
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let mut g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)<2 then return end
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local tg1=g:Select(tp,1,1,nil)
    let mut tg1 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // g:Remove(Card.IsCode,nil,tg1:GetFirst():GetCode())
    let first = group::get_first(d, &tg1).nil();
    let codes = card::get_code(d, first);
    group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local tg2=g:Select(tp,1,1,nil)
    let tg2 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // tg1:Merge(tg2)
    group::add_card(&mut tg1, &tg2);
    // if #tg1==2 then
    if tg1.len() == 2 {
        // Duel.SendtoExtraP(tg1,tp,REASON_EFFECT)
        duel::send_to_extra_p(d, tg1, Some(a.tp), REASON_EFFECT, None);
    }
}
