//! Delta Attacker
use super::prelude::*;

pub const ID: u32 = 39719977;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return c:IsFaceup() and (tpe&TYPE_NORMAL)~=0 and (tpe&TYPE_TOKEN)==0
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,2,c,c:GetCode())
    let code = card::get_code(d, c)[0];
    card::is_faceup(d, c)
        && (tpe & TYPE_NORMAL) != 0
        && (tpe & TYPE_TOKEN) == 0
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, code)), tp, LOCATION_MZONE, 0, 2, c.into())
}
/// `s.filter2(c,code)`
fn filter2(d: &mut Duel, c: CardId, code: u32) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return c:IsFaceup() and (tpe&TYPE_NORMAL)~=0 and (tpe&TYPE_TOKEN)==0 and c:IsCode(code)
    card::is_faceup(d, c) && (tpe & TYPE_NORMAL) != 0 && (tpe & TYPE_TOKEN) == 0 && card::is_code(d, c, &[code])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,3,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, 3, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
