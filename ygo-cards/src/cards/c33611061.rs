//! Bujinunity
use super::prelude::*;

pub const ID: u32 = 33611061;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BUJIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_HANDES+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_HANDES + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, 0, 1, h.into())
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_BUJIN as u16]) && card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_BUJIN as u16]) && card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil)
    let tg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        // Duel.SendtoGrave(hg,REASON_EFFECT)
        duel::send_to_grave(d, hg, REASON_EFFECT, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let mut g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g1=g:Select(tp,1,1,nil)
        let mut g1 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,g1:GetFirst():GetCode())
        let first = group::get_first(d, &g1).nil();
        let code = card::get_code(d, first)[0];
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &[code]), NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g2=g:Select(tp,1,1,nil)
            let g2 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // g:Remove(Card.IsCode,nil,g2:GetFirst():GetCode())
            let first2 = group::get_first(d, &g2).nil();
            let code2 = card::get_code(d, first2)[0];
            group::remove(d, &mut g, &|d, c| card::is_code(d, c, &[code2]), NO_EX);
            // g1:Merge(g2)
            group::add_card(&mut g1, &g2);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                // local g3=g:Select(tp,1,1,nil)
                let g3 = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
                // g1:Merge(g3)
                group::add_card(&mut g1, &g3);
            }
        }
        // Duel.SendtoHand(g1,nil,REASON_EFFECT)
        duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g1)
        duel::confirm_cards(d, 1 - tp, g1);
    }
}
