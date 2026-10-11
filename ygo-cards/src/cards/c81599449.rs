//! Chemicritter Carbo Crab
//! 化合獣カーボン・クラブ
use super::prelude::*;

pub const ID: u32 = 81599449;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Send 1 Gemini monster to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.tgfilter(c) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c)
    tgfilter(d, c) && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_GEMINI) and c:IsAbleToGrave()
    card::is_type(d, c, TYPE_GEMINI) && card::is_able_to_grave(d, c, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_GEMINI) and c:IsAbleToHand()
    card::is_type(d, c, TYPE_GEMINI) && card::is_able_to_hand(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoGrave(g,REASON_EFFECT)==0 or not g:GetFirst():IsLocation(LOCATION_GRAVE) then return end
    if g.is_empty() || duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_GRAVE) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local tg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let tg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - a.tp, tg);
    }
}
