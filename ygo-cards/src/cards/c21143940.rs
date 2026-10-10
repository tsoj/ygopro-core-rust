//! Mask Change
use super::prelude::*;

pub const ID: u32 = 21143940;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MASKED_HERO, SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tfilter(c,att,e,tp,mc)`: `mc` is nil in the calls that do not pass a card.
fn tfilter(d: &mut Duel, c: CardId, att: u32, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_MASKED_HERO) and c:IsAttribute(att) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    if !(card::is_set_card(d, c, &[SET_MASKED_HERO as u16]) && card::is_attribute(d, c, att)) {
        return false;
    }
    let ex: Exclude = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HERO)
    //     and Duel.IsExistingMatchingCard(s.tfilter,tp,LOCATION_EXTRA,0,1,nil,c:GetAttribute(),e,tp,c)
    if !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HERO as u16])) {
        return false;
    }
    let att = card::get_attribute(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| tfilter(d, x, att, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.chkfilter(c,att)`
fn chkfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_HERO) and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HERO as u16]) && card::is_attribute(d, c, att)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.chkfilter(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let att = effect::get_label(d, a.e) as u32;
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && chkfilter(d, chkc, att);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // e:SetLabel(g:GetFirst():GetAttribute())
    let tc = group::get_first(d, &g).nil();
    let att = card::get_attribute(d, tc);
    effect::set_label(d, a.e, &[att as i64]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local att=tc:GetAttribute()
    let att = card::get_attribute(d, tc);
    // if Duel.SendtoGrave(tc,REASON_EFFECT)==0 then return end
    if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.tfilter,tp,LOCATION_EXTRA,0,nil,att,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tfilter(d, c, att, a.e, tp, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, sg.clone(), 0, tp, tp, true, false, POS_FACEUP as u8, None);
        // sg:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &sg).nil();
        card::complete_procedure(d, first);
    }
}
