//! Tactical Exchanger
use super::prelude::*;

pub const ID: u32 = 58421530;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ROKKET}
    listed_series: &[SET_ROKKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster and Special Summon 1 "Rokket" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
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
// function s.desfilter(c,e,tp)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,c:GetOriginalCode())
    let code = card::get_original_code(d, c);
    card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,code)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_ROKKET) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsOriginalCode(code)
    card::is_set_card(d, c, &[SET_ROKKET as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_original_code(d, c, &[code])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc~=c and s.desfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && chkc != c && desfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,c,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,tc:GetOriginalCode())
        let code = card::get_original_code(d, tc);
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, code))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
