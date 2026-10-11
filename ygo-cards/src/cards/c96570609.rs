//! Ehther the Heavenly Monarch
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 96570609;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MONARCH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Tribute Summon this card by Tributing 1 Tribute Summoned monster
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),Card.IsTributeSummoned)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(tribute_summoned)), None);
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),Card.IsTributeSummoned)
    let _e2 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(Arc::new(tribute_summoned)), None);
    // Send 2 "Monarch" Spells/Traps with different names from your hand and/or Deck to the GY, and if you do, Special Summon 1 monster with 2400 or more ATK and 1000 DEF from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(sumcon)));
    // e2:SetCost(Cost.AND(Cost.SoftOncePerChain(id),s.sumcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::soft_once_per_chain(ID, None), cost_target(sumcost)])));
    // e2:SetTarget(s.sumtg)
    effect::set_target(d, e2, Some(cost_target(sumtg)));
    // e2:SetOperation(s.sumop)
    effect::set_operation(d, e2, Some(operation(sumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
/// `Card.IsTributeSummoned` as the tribute filter
fn tribute_summoned(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::is_tribute_summoned(d, c)
}
/// `function(e) return e:GetHandler():IsTributeSummoned() end`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MONARCH) and c:IsSpellTrap() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_MONARCH as u16]) && card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttackAbove(2400) and c:IsDefense(1000) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attack_above(d, c, 2400) && card::is_defense(d, c, &[1000]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>1
        if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) <= 1 {
            return false;
        }
        //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.delayed` operation: `function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end`
fn delayed_back(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_TOGRAVE)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    // if #tg~=2 or Duel.SendtoGrave(tg,REASON_EFFECT)==0
    if tg.len() != 2 {
        return;
    }
    if duel::send_to_grave(d, tg.clone(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    //   or not tg:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE)
    if !group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX) {
        return;
    }
    //   or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Return it to the hand during the End Phase
            // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(ag) Duel.SendtoHand(ag,nil,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,3))
            aux::delayed_operation(
                d,
                tc,
                PHASE_END,
                ID,
                a.e,
                tp,
                Some(Arc::new(delayed_back)),
                None,
                Some(0),
                Some(0),
                Some(aux::stringid(ID, 3)),
                None,
            );
        }
    }
}
/// `s.sumcostfilter(c)`
fn sumcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MONARCH) and c:IsSpellTrap() and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_MONARCH as u16]) && card::is_spell_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.sumcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumcostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumcostfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sumcostfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&sumcostfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CanSummonOrSet(true,nil,1) end
    if a.chk == Some(0) {
        return card::can_summon_or_set(d, c, true, None, 1, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SummonOrSet(tp,c,true,nil,1)
        duel::summon_or_set(d, a.tp, c, true, None, 1, None);
    }
}
