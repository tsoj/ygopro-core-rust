//! Evoltile Elginero
use super::prelude::*;

pub const ID: u32 = 91903221;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EVOLTILE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card, shuffle 1 Dinousaur monster into the Deck, and search 1 "Evoltile" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_RELEASE)
    effect::set_code(d, e1, EVENT_RELEASE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsLocation(LOCATION_GRAVE)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsAbleToDeck()
    card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_deck(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVOLTILE) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EVOLTILE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_HAND,0,nil)
    let g1 = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g2 = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g1>0 and #g2>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g1.is_empty() && !g2.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local tg1=g1:Select(tp,1,1,nil)
        let tg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,tg1)
        duel::confirm_cards(d, 1 - tp, tg1.clone());
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SendtoDeck(tg1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
        if duel::send_to_deck(d, tg1, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local tg2=g2:Select(tp,1,1,nil)
        let tg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(tg2,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg2.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg2)
        duel::confirm_cards(d, 1 - tp, tg2);
    }
}
