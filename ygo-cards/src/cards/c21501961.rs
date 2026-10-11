//! Pair Bear Scare!!
use super::prelude::*;

pub const ID: u32 = 21501961;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent can reveal 1 "Pair Bear Scare!!" in their hand or Deck, and if they do, each player gains 2000 LP, otherwise, you destroy 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card to your opponent's hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_ONFIELD)
    })));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,PLAYER_ALL,2000)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, PLAYER_ALL as u8, 2000);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// s.revealfilter(c)
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and not c:IsPublic()
    card::is_code(d, c, &[ID]) && !card::is_public(d, c)
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local opp=1-tp
    let opp = 1 - tp;
    // if Duel.IsExistingMatchingCard(s.revealfilter,opp,LOCATION_HAND|LOCATION_DECK,0,1,nil)
    //   and Duel.SelectYesNo(opp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&revealfilter), opp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, opp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_CONFIRM as u64);
        // local sc=Duel.SelectMatchingCard(opp,s.revealfilter,opp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
        let sg = duel::select_matching_card(d, opp, Some(&revealfilter), opp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        // Duel.ConfirmCards(tp,sc)
        duel::confirm_cards(d, tp, sc);
        // if sc:IsLocation(LOCATION_HAND) then
        if card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ShuffleHand(opp)
            duel::shuffle_hand(d, opp);
        } else {
            // Duel.ShuffleDeck(opp)
            duel::shuffle_deck(d, opp);
        }
        // Duel.Recover(tp,2000,REASON_EFFECT)
        duel::recover(d, tp, 2000, REASON_EFFECT, false, None);
        // Duel.Recover(opp,2000,REASON_EFFECT)
        duel::recover(d, opp, 2000, REASON_EFFECT, false, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,1-tp,REASON_EFFECT)
        duel::send_to_hand(d, c, Some(1 - a.tp), REASON_EFFECT, None);
    }
}
