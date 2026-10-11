//! Northern Crossfire
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 79936051;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If the total Levels of monsters you control are 10 or higher: Take 1 Level 10 monster from your hand or Deck, and either add it to your hand or Special Summon it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil):GetSum(Card.GetLevel)>=10 end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If the total Levels of monsters your opponent controls are 10 or higher: You can banish this card and 1 Level 10 monster from your GY, then target 1 card your opponent controls; banish it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetSum(Card.GetLevel)>=10 end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetCost(s.bancost)
    effect::set_cost(d, e2, Some(cost_target(bancost)));
    // e2:SetTarget(s.bantg)
    effect::set_target(d, e2, Some(cost_target(bantg)));
    // e2:SetOperation(s.banop)
    effect::set_operation(d, e2, Some(operation(banop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thspfilter(c,e,tp,sp_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sp_chk: bool) -> bool {
    // return c:IsLevel(10) and (c:IsAbleToHand() or (sp_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_level(d, c, &[10])
        && (card::is_able_to_hand(d, c, None)
            || (sp_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `Duel.GetLocationCount(tp,LOCATION_MZONE)>0`
fn sp_check(d: &mut Duel, tp: u8) -> bool {
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `function(e,tp) ... GetSum(Card.GetLevel)>=10 end` of e1
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_sum(d, &g, &|d, c| card::get_level(d, c) as i64) >= 10
}
/// `function(e,tp) ... GetSum(Card.GetLevel)>=10 end` of e2
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    group::get_sum(d, &g, &|d, c| card::get_level(d, c) as i64) >= 10
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local sp_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let sp = sp_check(d, tp);
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,sp_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, tp, sp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sp_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let sp = sp_check(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp,sp_chk):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, a.e, tp, sp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &sel) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // aux.ToHandOrElse(sc,tp,
    //     function() return sp_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    let check = |d: &mut Duel, _c: CardId| sp && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None);
    let oper = |d: &mut Duel, _g: &CardSet| -> i32 {
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        0
    };
    aux::to_hand_or_else(d, sc, tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 3)));
}
/// `s.bancostfilter(c)`
fn bancostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(10) and c:IsAbleToRemoveAsCost()
    card::is_level(d, c, &[10]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.bancost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bancost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.bancostfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&bancostfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.bancostfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let mut g = duel::select_matching_card(d, tp, Some(&bancostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
/// `s.banop(e,tp,eg,ep,ev,re,r,rp)`
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
