//! Form Change
use super::prelude::*;

pub const ID: u32 = 84536654;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MASKED_HERO, SET_HERO], ..ScriptInfo::DEFAULT };

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
/// `s.spfilter(c,code,lv,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, code: u32, lv: u32, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:GetLevel()==lv and c:IsSetCard(SET_MASKED_HERO) and not c:IsCode(code)
    //   and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::get_level(d, c) == lv
        && card::is_set_card(d, c, &[SET_MASKED_HERO as u16])
        && !card::is_code(d, c, &[code])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, |m| m.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HERO) and c:IsType(TYPE_FUSION) and c:IsAbleToExtra()
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,c:GetCode(),c:GetOriginalLevel(),e,tp,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_HERO as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_able_to_extra(d, c)
        && {
            let code = card::get_code(d, c)[0];
            let lv = card::get_original_level(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, code, lv, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    // local lv=tc:GetOriginalLevel()
    let lv = card::get_original_level(d, tc);
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)==0 then return end
    if duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,code,lv,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, code, lv, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g.clone(), 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &g).nil();
        card::complete_procedure(d, first);
    }
}
