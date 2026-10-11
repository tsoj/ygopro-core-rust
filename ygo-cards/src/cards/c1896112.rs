//! Dimension Explosion
use super::prelude::*;

pub const ID: u32 = 1896112;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,ft)`: `return c:IsFaceup() and c:IsType(TYPE_FUSION) and c:IsAbleToExtraAsCost() and (ft>0 or c:GetSequence()<5)`
fn cfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_able_to_extra_as_cost(d, c)
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,ft) end
    if a.chk == Some(0) {
        return ft > -1 && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,ft)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp)`: `return c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_REMOVED,0,1,nil,e,tp)
    //   and Duel.IsExistingTarget(s.filter,1-tp,LOCATION_REMOVED,0,1,nil,e,1-tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft1=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft1 = 1;
    }
    // if ft1>2 then ft1=2 end
    if ft1 > 2 {
        ft1 = 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.filter,tp,LOCATION_REMOVED,0,1,ft1,nil,e,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, ft1 as u16, false, NO_EX).nil();
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let mut ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft2>2 then ft2=2 end
    if ft2 > 2 {
        ft2 = 2;
    }
    // if Duel.IsPlayerAffectedByEffect(1-tp,CARD_BLUEEYES_SPIRIT) then ft2=1 end
    if duel::is_player_affected_by_effect(d, 1 - tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft2 = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(1-tp,s.filter,1-tp,LOCATION_REMOVED,0,1,ft2,nil,e,1-tp)
    let g2 = duel::select_target(d, 1 - tp, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_REMOVED, 0, 1, ft2 as u16, false, NO_EX).nil();
    // g1:Merge(g2)
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,#g1,0,0)
    let n = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local g1=g:Filter(Card.IsControler,nil,tp)
    let g1 = group::filter(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // local g2=g:Filter(Card.IsControler,nil,1-tp)
    let g2 = group::filter(d, &g, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ct1=#g1
    let ct1 = g1.len() as i32;
    // if ft1>=ct1 and (ct1==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then
    if ft1 >= ct1 && (ct1 == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()) {
        // local tc=g1:GetFirst()
        let _ = group::get_first(d, &g1);
        // for tc in aux.Next(g1) do
        for tc in group::iter(d, &g1) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ct2=#g2
    let ct2 = g2.len() as i32;
    // if ft2>=ct2 and (ct2==1 or not Duel.IsPlayerAffectedByEffect(1-tp,CARD_BLUEEYES_SPIRIT)) then
    if ft2 >= ct2 && (ct2 == 1 || duel::is_player_affected_by_effect(d, 1 - tp, CARD_BLUEEYES_SPIRIT).is_none()) {
        // local tc=g2:GetFirst()
        let _ = group::get_first(d, &g2);
        // for tc in aux.Next(g2) do
        for tc in group::iter(d, &g2) {
            // Duel.SpecialSummonStep(tc,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
