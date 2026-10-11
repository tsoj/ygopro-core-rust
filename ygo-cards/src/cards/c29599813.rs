//! Purrely Pretty Memory
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 29599813;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PURRELY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each player gains 1000 LP, discard 1 card and Special Summon 1 "Purrely" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Provide an effect to a "Purrely" Xyz monster with this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsSetCard(SET_PURRELY) end)
    effect::set_condition(d, e2, Some(condition(atch_cond)));
    // e2:SetCost(s.atchcost)
    effect::set_cost(d, e2, Some(cost_target(atchcost)));
    // e2:SetTarget(s.atchtg)
    effect::set_target(d, e2, Some(cost_target(atchtg)));
    // e2:SetOperation(s.atchop)
    effect::set_operation(d, e2, Some(operation(atchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsLevel(1) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_level(d, c, &[1])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,PLAYER_ALL,1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, PLAYER_ALL as u8, 1000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.Recover(tp,1000,REASON_EFFECT)==0 or Duel.Recover(1-tp,1000,REASON_EFFECT)==0 then return end
    if duel::recover(d, tp, 1000, REASON_EFFECT, false, None) == 0 {
        return;
    }
    if duel::recover(d, 1 - tp, 1000, REASON_EFFECT, false, None) == 0 {
        return;
    }
    // Discard 1 card and Special Summon 1 "Purrely" monster from the Deck
    // if Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_EFFECT|REASON_DISCARD,nil)>0 then
        if duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
// function s.atchcostfilter(c,tp)
fn atchcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and Duel.IsExistingTarget(s.atchfilter,tp,0,LOCATION_ONFIELD,1,c:GetEquipGroup()+c)
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    let mut ex = card::get_equip_group(d, c);
    group::add_card(&mut ex, &CardSet::from_card(c));
    duel::is_existing_target(d, Some(&atchfilter), tp, 0, LOCATION_ONFIELD, 1, (&ex).into())
}
// function s.atchfilter(c)
fn atchfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOKEN) and c:IsAbleToChangeControler()
    !card::is_type(d, c, TYPE_TOKEN) && card::is_able_to_change_controler(d, c)
}
// function s.atchcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atchcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atchcostfilter,tp,LOCATION_ONFIELD,0,1,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| atchcostfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atchcostfilter,tp,LOCATION_ONFIELD,0,1,1,c,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| atchcostfilter(d, x, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
// function s.atchtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(1-tp) and s.atchfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, 1 - tp) && atchfilter(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(s.atchfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::is_existing_target(d, Some(&atchfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atchfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    duel::select_target(d, tp, Some(&atchfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
// function s.atchop(e,tp,eg,ep,ev,re,r,rp)
fn atchop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e) {
            // tc:CancelToGrave()
            card::cancel_to_grave(d, tc, true);
            // Duel.Overlay(c,tc,true)
            duel::overlay(d, c, tc, true);
        }
    }
}
fn atch_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(e) return e:GetHandler():IsSetCard(SET_PURRELY) end
    let c = effect::get_handler(d, a.e).nil();
    card::is_set_card(d, c, &[SET_PURRELY as u16])
}
