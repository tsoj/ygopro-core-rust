//! Cyber Revsystem
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 33041277;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Cyber Dragon" from hand or GY
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
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
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_CYBER_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_CYBER_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(move |d, c| filter(d, c, e, tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if !g.is_empty() && duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // Cannot be destroyed by card effects
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3001)
        effect::set_description(d, e1, 3001);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // g:GetFirst():RegisterEffect(e1)
        let tc = group::get_first(d, &g).nil();
        card::register_effect(d, tc, e1, false);
    }
}
