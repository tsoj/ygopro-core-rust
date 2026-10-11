//! Dark World Puppetry
//! 暗黒界の傀儡
use super::prelude::*;

pub const ID: u32 = 30284022;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish up to 3 cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 banished Fiend monster to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemove(tp) and aux.SpElimFilter(c,false)
    card::is_able_to_remove(d, c, Some(tp), None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsDiscardable(REASON_EFFECT)
    card::is_race(d, c, RACE_FIEND) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=LOCATION_GRAVE|LOCATION_MZONE
    let loc = LOCATION_GRAVE | LOCATION_MZONE;
    // if chkc then return chkc:IsLocation(loc) and s.rmfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, loc) && rmfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,loc,loc,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, tp)), tp, loc, loc, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,loc,loc,1,3,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| rmfilter(d, c, tp)), tp, loc, loc, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 and Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)>0
    //     and tg:IsExists(Card.IsLocation,1,nil,LOCATION_REMOVED) then
    if !tg.is_empty()
        && duel::remove(d, tg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_REMOVED), 1, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local dg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND,0,1,1,nil)
        let dg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FIEND) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_REMOVED) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
