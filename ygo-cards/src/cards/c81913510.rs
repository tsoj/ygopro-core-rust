//! NEX
use super::prelude::*;

pub const ID: u32 = 81913510;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_NEO_SPACIAN}
    listed_series: &[SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local code=c:GetCode()
    let code = card::get_code(d, c)[0];
    // return c:IsFaceup() and c:IsSetCard(SET_NEO_SPACIAN)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,code,e,tp,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, code, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.filter2(c,code,e,tp,mc)
fn filter2(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsCode(code) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[code])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter1(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter1(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| filter1(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc1=Duel.GetFirstTarget()
    let tc1 = duel::first_target(d);
    // if tc1 and tc1:IsRelateToEffect(e) and Duel.SendtoGrave(tc1,REASON_EFFECT)~=0 and tc1:IsLocation(LOCATION_GRAVE) then
    if let Some(tc1) = tc1 {
        if card::is_relate_to_effect(d, tc1, a.e)
            && duel::send_to_grave(d, tc1, REASON_EFFECT, None, None) != 0
            && card::is_location(d, tc1, LOCATION_GRAVE)
        {
            // local code=tc1:GetCode()
            let code = card::get_code(d, tc1)[0];
            // local tc2=Duel.GetFirstMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,nil,code,e,tp,tc1)
            let tc2 = duel::get_first_matching_card(d, Some(&|d, x| filter2(d, x, code, a.e, tp, tc1)), tp, LOCATION_EXTRA, 0, NO_EX);
            // if tc2 and Duel.SpecialSummon(tc2,0,tp,tp,true,false,POS_FACEUP)~=0 then
            if let Some(tc2) = tc2 {
                if duel::special_summon(d, tc2, 0, tp, tp, true, false, POS_FACEUP as u8, None) != 0 {
                    // tc2:CompleteProcedure()
                    card::complete_procedure(d, tc2);
                }
            }
        }
    }
}
