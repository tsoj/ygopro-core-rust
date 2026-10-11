//! Generation Shift
use super::prelude::*;

pub const ID: u32 = 34460239;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 face-up monster you control, then add 1 card with the same name as the destroyed card from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.desfilter(c,tp)
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode())
    // (the extra argument is every code of c; the filter only reads the first)
    if !card::is_faceup(d, c) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.thfilter(c,code)
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToHand()
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.desfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && desfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsControler(tp) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_faceup(d, tc)
        && card::is_relate_to_effect(d, tc, e)
        && card::is_controler(d, tc, tp)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetPreviousCodeOnField())
        let code = card::get_previous_code_on_field(d, tc).first().copied().nil();
        let g = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
