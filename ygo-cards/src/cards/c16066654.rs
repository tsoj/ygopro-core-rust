//! Ｒ．Ｂ．オペレーション・テスト
//! R.B. Operation Test
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 16066654;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can target 1 Level 3 or higher "R.B." monster in your GY; Special Summon it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
    // Gain LP equal to the combined original ATK of the targeted "R.B." monsters, and if you do, return them to the hand/Extra Deck, then you can Special Summon 1 "R.B." monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RECOVER+CATEGORY_TOHAND+CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RECOVER + CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.recthsptg)
    effect::set_target(d, e2, Some(cost_target(recthsptg)));
    // e2:SetOperation(s.recthspop)
    effect::set_operation(d, e2, Some(operation(recthspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfromgyfilter(c,e,tp)`
fn spfromgyfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(3) and c:IsSetCard(SET_RB) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 3) && card::is_set_card(d, c, &[SET_RB as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfromgyfilter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(s.spfromgyfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_target(d, Some(&|d, x| spfromgyfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.spfromgyfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, x| spfromgyfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.recthfilter(c)`
fn recthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsFaceup() and (c:IsAbleToHand() or c:IsAbleToExtra())
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_faceup(d, c) && (card::is_able_to_hand(d, c, None) || card::is_able_to_extra(d, c))
}
/// `s.recthsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn recthsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && recthfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&recthfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local ct=Duel.GetTargetCount(s.recthfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_target_count(d, Some(&recthfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.recthfilter,tp,LOCATION_MZONE,0,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&recthfilter), tp, LOCATION_MZONE, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,g:GetSum(Card.GetBaseAttack))
    let sum = group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, sum as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spfromhandfilter(c,e,tp)`
fn spfromhandfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.recthspop(e,tp,eg,ep,ev,re,r,rp)`
fn recthspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, x| card::is_faceup(d, x), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local sum=tg:GetSum(Card.GetBaseAttack)
    let sum = group::get_sum(d, &tg, &|d, x| card::get_base_attack(d, x) as i64);
    // if Duel.Recover(tp,sum,REASON_EFFECT)>0 and Duel.SendtoHand(tg,nil,REASON_EFFECT)>0 and Duel.GetMZoneCount(tp)>0
    //     and tg:FilterCount(Card.IsLocation,nil,LOCATION_HAND|LOCATION_EXTRA)>0
    //     and Duel.IsExistingMatchingCard(s.spfromhandfilter,tp,LOCATION_HAND,0,1,nil,e,tp) then
    if duel::recover(d, tp, sum, REASON_EFFECT, false, None) > 0
        && duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None) > 0
        && duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0
        && group::filter_count(d, &tg, &|d, x| card::is_location(d, x, LOCATION_HAND | LOCATION_EXTRA), NO_EX) > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfromhandfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
    {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
        if !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfromhandfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfromhandfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
