//! World Legacy Bestowal
//! 星遺物の選託
use super::prelude::*;

pub const ID: u32 = 59778096;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WORLD_LEGACY) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsAbleToHand()
    card::is_race(d, c, RACE_CYBERSE as u64) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(Card.IsType,tp,0,LOCATION_MZONE,1,nil,TYPE_LINK)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,7,nil) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 7, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)]);
    // else
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) + 1;
    }
    // if op==0 then
    if op == 0 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // e:SetOperation(s.gyop)
        effect::set_operation(d, a.e, Some(operation(gyop)));
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    // else
    } else {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,7,7,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, 7, 7, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsType,tp,0,LOCATION_MZONE,1,1,nil,TYPE_LINK)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_type(d, c, TYPE_LINK)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
