//! Double Cipher
use super::prelude::*;

pub const ID: u32 = 63992027;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY_EYES, SET_CIPHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return false end
    if g.is_empty() {
        return false;
    }
    // local tg=g:GetMaxGroup(Card.GetAttack)
    let tg = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|r| r.0).unwrap_or_default();
    // return tg:IsExists(Card.IsControler,1,nil,1-tp)
    group::is_exists(d, &tg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX)
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard({SET_GALAXY_EYES,SET_CIPHER}) and c:GetOverlayCount()>0
    //     and (not e or Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c))
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_set_card(d, c, &[SET_GALAXY_EYES as u16, SET_CIPHER as u16])
        && card::get_overlay_count(d, c) > 0
        && match e {
            None => true,
            Some(e) => duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX),
        }
}
// function s.spfilter(c,e,tp,ec)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // return c:IsCode(ec:GetCode()) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let codes = card::get_code(d, ec);
    card::is_code(d, c, &codes)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,nil,nil) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, None, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, Some(a.e), a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, Some(a.e), a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local og=tc:GetOverlayGroup()
    let og = card::get_overlay_group(d, tc);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // if Duel.SendtoGrave(og,REASON_EFFECT)~=0 then
    if duel::send_to_grave(d, og, REASON_EFFECT, None, None) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, tc)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
