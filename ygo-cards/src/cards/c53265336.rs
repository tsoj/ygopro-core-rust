//! Solfachord Scale
use super::prelude::*;

pub const ID: u32 = 53265336;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_PENDULUM) and c:IsOriginalType(TYPE_MONSTER) and c:IsSetCard(SET_SOLFACHORD)
    card::is_faceup(d, c)
        && card::is_original_type(d, c, TYPE_PENDULUM)
        && card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)>2
    duel::get_matching_group_count(d, Some(&|d, c| filter(d, c)), a.tp, LOCATION_ONFIELD, 0, NO_EX) > 2
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_SOLFACHORD) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_PENDULUM)
        && card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `g:GetClassCount(Card.GetCode)` over the face-up Pendulum monsters `s.filter` finds on the field.
fn class_count(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64)
}
/// `b1` of the Lua: a card to add to the hand from the Pendulum Zone, a Pendulum in the Extra Deck, 3 classes.
fn b1_of(d: &mut Duel, tp: u8, ct: usize) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_PZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && ct >= 3
}
/// `b2` of the Lua: a free Monster Zone, a Solfachord Pendulum in the hand, 5 classes.
fn b2_of(d: &mut Duel, e: EffectId, tp: u8, ct: usize) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && ct >= 5
}
/// `b3` of the Lua: an opponent's face-up card, 7 classes.
fn b3_of(d: &mut Duel, tp: u8, ct: usize) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX) && ct >= 7
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,nil)
    // local ct=g:GetClassCount(Card.GetCode)
    let ct = class_count(d, tp);
    // local b1=...
    let b1 = b1_of(d, tp, ct);
    // local b2=...
    let b2 = b2_of(d, e, tp, ct);
    // local b3=...
    let b3 = b3_of(d, tp, ct);
    // if chk==0 then return (b1 or b2 or b3) end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,nil)
    // local ct=g:GetClassCount(Card.GetCode)
    let mut ct = class_count(d, tp);
    // local b1=...
    let b1 = b1_of(d, tp, ct);
    // local b2=...
    let mut b2 = b2_of(d, e, tp, ct);
    // local b3=...
    let mut b3 = b3_of(d, tp, ct);
    // local flag=0
    let mut flag = 0;
    // if b1 and ((not b2 and not b3) or Duel.SelectYesNo(tp,aux.Stringid(id,0))) then
    if b1 && ((!b2 && !b3) || duel::select_yes_no(d, tp, aux::stringid(ID, 0))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local thg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_PZONE,0,1,1,nil)
        let thg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
        // if Duel.SendtoHand(thg,tp,REASON_EFFECT)>0 and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_HAND) then
        if duel::send_to_hand(d, thg, Some(tp), REASON_EFFECT, None) > 0 {
            let operated = duel::get_operated_group(d);
            let first = group::get_first(d, &operated).nil();
            if card::is_location(d, first, LOCATION_HAND) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
                // local tpg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil)
                let tpg = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
                // if #tpg>0 then
                if !tpg.is_empty() {
                    // Duel.MoveToField(tpg:GetFirst(),tp,tp,LOCATION_PZONE,POS_FACEUP,true)
                    let tc = group::get_first(d, &tpg).nil();
                    duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
                }
            }
        }
        // flag=1
        flag = 1;
        // g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,nil)
        // ct=g:GetClassCount(Card.GetCode)
        ct = class_count(d, tp);
        // b2=...
        b2 = b2_of(d, e, tp, ct);
        // b3=...
        b3 = b3_of(d, tp, ct);
    }
    // if b2 and ((flag==0 and not b3) or Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if b2 && ((flag == 0 && !b3) || duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // flag=1
        flag = 1;
        // g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,nil)
        // ct=g:GetClassCount(Card.GetCode)
        ct = class_count(d, tp);
        // b3=...
        b3 = b3_of(d, tp, ct);
    }
    // if b3 and (flag==0 or Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    if b3 && (flag == 0 || duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
