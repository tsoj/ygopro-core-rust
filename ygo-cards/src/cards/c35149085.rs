//! Beast Soul Swap
use super::prelude::*;

pub const ID: u32 = 35149085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,ft)
fn filter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:HasLevel() and c:IsRace(RACE_BEAST) and c:IsAbleToHand() and (ft>0 or c:GetSequence()<5)
    card::is_faceup(d, c)
        && card::has_level(d, c)
        && card::is_race(d, c, RACE_BEAST)
        && card::is_able_to_hand(d, c, None)
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
// function s.spfilter(c,e,tp,lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsRace(RACE_BEAST) and c:GetLevel()==lv and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_BEAST)
        && card::get_level(d, c) == lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, ft);
    }
    // if chk==0 then return ft>-1 and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,ft) end
    if a.chk == Some(0) {
        return ft > -1 && duel::is_existing_target(d, Some(&|d, c| filter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,ft)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local lv=tc:GetLevel()
            let lv = card::get_level(d, tc);
            // if Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND)
            //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
            if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
                && card::is_location(d, tc, LOCATION_HAND)
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,lv)
                let e = a.e;
                let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
