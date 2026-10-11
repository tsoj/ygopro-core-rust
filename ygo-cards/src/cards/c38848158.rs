//! Igknights Unite
use super::prelude::*;

pub const ID: u32 = 38848158;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter1(c,ft)`
fn desfilter1(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGKNIGHT) and (ft>0 or (c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5))
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_IGKNIGHT as u16])
        && (ft > 0 || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_IGKNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_IGKNIGHT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc~=c and s.desfilter1(chkc,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && chkc != c && desfilter1(d, chkc, ft);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // return ft>-1 and Duel.IsExistingTarget(s.desfilter1,tp,LOCATION_ONFIELD,0,1,c,ft)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        return ft > -1
            && duel::is_existing_target(d, Some(&|d, x| desfilter1(d, x, ft)), a.tp, LOCATION_ONFIELD, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter1,tp,LOCATION_ONFIELD,0,1,1,c,ft)
    let g = duel::select_target(d, a.tp, Some(&|d, x| desfilter1(d, x, ft)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e)
            && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
