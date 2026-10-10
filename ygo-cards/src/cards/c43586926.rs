//! Twin-Headed Behemoth
use super::prelude::*;

pub const ID: u32 = 43586926;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to grave
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_DESTROY) and (c:GetPreviousLocation()&LOCATION_ONFIELD)~=0 then
    if card::is_reason(d, c, REASON_DESTROY) && (card::get_previous_location(d, c) & LOCATION_ONFIELD) != 0 {
        // spsummon
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        // e1:SetRange(LOCATION_GRAVE)
        effect::set_range(d, e1, LOCATION_GRAVE);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
        effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
        // e1:SetTarget(s.sptg)
        effect::set_target(d, e1, Some(cost_target(sptg)));
        // e1:SetOperation(s.spop)
        effect::set_operation(d, e1, Some(operation(spop)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
