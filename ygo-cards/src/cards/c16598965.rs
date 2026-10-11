//! Stained Glass of Light & Dark
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 16598965;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TODECK+CATEGORY_HANDES+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK + CATEGORY_HANDES + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsRace(RACE_FAIRY|RACE_FIEND)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_race(d, c, RACE_FAIRY | RACE_FIEND)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0
    //     and ((g:IsExists(Card.IsRace,1,nil,RACE_FAIRY) and Duel.IsPlayerCanDraw(tp,3))
    //     or (g:IsExists(Card.IsRace,1,nil,RACE_FIEND) and Duel.IsPlayerCanDraw(1-tp,1))) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && ((group::is_exists(d, &g, &|d, c| card::is_race(d, c, RACE_FAIRY), 1, NO_EX) && duel::is_player_can_draw(d, tp, 3))
                || (group::is_exists(d, &g, &|d, c| card::is_race(d, c, RACE_FIEND), 1, NO_EX) && duel::is_player_can_draw(d, 1 - tp, 1)));
    }
    // if g:IsExists(Card.IsRace,1,nil,RACE_FAIRY) then
    if group::is_exists(d, &g, &|d, c| card::is_race(d, c, RACE_FAIRY), 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,3)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 3);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, tp, LOCATION_HAND as i32);
    }
    // if g:IsExists(Card.IsRace,1,nil,RACE_FIEND) then
    if group::is_exists(d, &g, &|d, c| card::is_race(d, c, RACE_FIEND), 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, 1);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local fg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let fg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
    // if fg:IsExists(Card.IsRace,1,nil,RACE_FAIRY) and Duel.Draw(tp,3,REASON_EFFECT)==3 then
    if group::is_exists(d, &fg, &|d, c| card::is_race(d, c, RACE_FAIRY), 1, NO_EX) && duel::draw(d, tp, 3, REASON_EFFECT) == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,2,2,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
            // Duel.SortDeckbottom(tp,tp,2)
            duel::sort_deckbottom(d, tp, tp, 2);
        }
    }
    // if fg:IsExists(Card.IsRace,1,nil,RACE_FIEND) then
    if group::is_exists(d, &fg, &|d, c| card::is_race(d, c, RACE_FIEND), 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.Draw(1-tp,1,REASON_EFFECT)>0 then
        if duel::draw(d, 1 - tp, 1, REASON_EFFECT) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local dg=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0):RandomSelect(1-tp,1)
            let hand = duel::get_field_group(d, 1 - tp, LOCATION_HAND, 0);
            let dg = group::random_select(d, &hand, 1 - tp, 1);
            // if Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)>0 and Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)>0 then
            if duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None) > 0 && duel::get_field_group_count(d, 1 - tp, LOCATION_HAND, 0) > 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
                // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
                duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
            }
        }
    }
}
