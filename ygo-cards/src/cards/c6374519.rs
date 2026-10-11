//! Amazement Special Show
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 6374519;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 of your targeted "Amazement" monsters to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZEMENT) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsControler(tp) and (not e or c:IsCanBeEffectTarget(e)) and c:IsAbleToHand()
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_AMAZEMENT as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
        && card::is_able_to_hand(d, c, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_AMAZEMENT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_AMAZEMENT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // return tg and tg:IsExists(s.thfilter,1,nil,tp,e)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let Some(tg) = tg.first().and_then(|v| v.as_group()).cloned() else { return false };
    let tp = a.tp;
    group::is_exists(d, &tg, &|d, c| thfilter(d, c, tp, Some(a.e)), 1, NO_EX)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = g.first().and_then(|v| v.as_group()).cloned();
    // if chkc then return eg:IsContains(chkc) and s.thfilter(chkc,tp,e) end
    if let Some(chkc) = a.chkc() {
        let contains = a.eg.as_ref().is_some_and(|eg| group::is_contains(eg, chkc));
        return contains && thfilter(d, chkc, a.tp, Some(a.e));
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local sg=g:FilterSelect(tp,s.thfilter,1,1,nil,tp,e)
    let tp = a.tp;
    let sg = group::filter_select(d, &g.nil(), tp, &|d, c| thfilter(d, c, tp, Some(a.e)), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 then
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_hand(d, tc, None, REASON_EFFECT, None) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !g.is_empty()
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
