//! Mask Change II
use super::prelude::*;

pub const ID: u32 = 93600443;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MASKED_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, h) = (a.tp, effect::get_handler(d, a.e).nil());
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // local att=c:GetAttribute()
    let att = card::get_attribute(d, c);
    // return lv>0 and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,lv,att,c)
    lv > 0
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, lv, att, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32, att: u32, mc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_MASKED_HERO) and c:GetLevel()>lv and c:IsAttribute(att)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_MASKED_HERO as u16])
        && card::get_level(d, c) > lv
        && card::is_attribute(d, c, att)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn chkfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // local att=tc:GetAttribute()
    let att = card::get_attribute(d, tc);
    // return c:IsFaceup() and c:IsLevelBelow(lv) and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_level_below(d, c, lv) && card::is_attribute(d, c, att)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.chkfilter(chkc,e:GetLabelObject()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label_object(d, e).as_card().nil();
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && chkfilter(d, chkc, label);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g.nil());
    effect::set_label_object(d, e, Value::Card(first.nil()));
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local att=tc:GetAttribute()
    let att = card::get_attribute(d, tc);
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // if Duel.SendtoGrave(tc,REASON_EFFECT)==0 then return end
    if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv,att)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv, att, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g.clone(), 0, tp, tp, true, false, POS_FACEUP as u8, None);
        // g:GetFirst():CompleteProcedure()
        let first = group::get_first(d, &g).nil();
        card::complete_procedure(d, first);
    }
}
