//! Mermail Abyssocea
use super::prelude::*;

pub const ID: u32 = 28577986;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MERMAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,e,tp)
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsFaceup() and c:IsSetCard(SET_MERMAIL)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,lv,e,tp)
    lv > 0
        && card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MERMAIL as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, lv, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.spfilter(c,lv,e,tp)
fn spfilter(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    // if lv>4 then lv=4 end
    let lv = if lv > 4 { 4 } else { lv };
    // return c:IsLevelBelow(lv) and c:IsSetCard(SET_MERMAIL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, lv)
        && card::is_set_card(d, c, &[SET_MERMAIL as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| cfilter(d, x, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| cfilter(d, x, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.rescon(lv)
//   return function(sg,e,tp,mg) return sg:GetSum(Card.GetLevel)<=lv end
// end
fn rescon(lv: u32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| (group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) <= lv as i64, false)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local slv=tc:GetLevel()
    let slv = card::get_level(d, tc);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,slv,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, slv, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local tg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.rescon(slv),1,tp,HINTMSG_SPSUMMON)
    let tg = aux::select_unselect_group(
        d,
        &sg,
        Some(e),
        tp,
        Some(1),
        Some(ft as usize),
        Some(&rescon(slv)),
        1,
        tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoGrave(tc,REASON_EFFECT)
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
}
