//! Speedroid Wheel
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 12148078;

// s.roll_dice=true
// s.listed_series={SET_SPEEDROID}
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DICE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, None, None)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, None, None)), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:HasLevel() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SPEEDROID as u16])
        && card::has_level(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, tp, 1, 0)[0];
    // local flag=false
    let mut flag = false;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),2)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:SelectWithSumEqual(tp,Card.GetLevel,dc,1,ft)
        let sg = group::select_with_sum_equal(d, &g, tp, &|d, x| card::get_level(d, x) as i64, dc as u32, 1, ft).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // for tc in sg:Iter() do
            for tc in group::iter(d, &sg) {
                // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
                if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                    // local e1=Effect.CreateEffect(e:GetHandler())
                    let h = effect::get_handler(d, e).nil();
                    let e1 = effect::create_effect(d, h);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_DISABLE)
                    effect::set_code(d, e1, EFFECT_DISABLE);
                    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                    // local e2=e1:Clone()
                    let e2 = effect::clone(d, e1);
                    // e2:SetCode(EFFECT_DISABLE_EFFECT)
                    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                    // tc:RegisterEffect(e2)
                    card::register_effect(d, tc, e2, false);
                    // flag=true
                    flag = true;
                }
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
    // if not flag then Duel.SetLP(tp,Duel.GetLP(tp)-dc*500) end
    if !flag {
        let lp = duel::get_lp(d, tp) - dc * 500;
        duel::set_lp(d, tp, lp);
    }
}
