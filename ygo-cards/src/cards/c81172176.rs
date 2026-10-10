//! Fiend Comedian
use super::prelude::*;

pub const ID: u32 = 81172176;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and banish cards from the opponent's GY or send cards from Deck to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COIN+CATEGORY_REMOVE+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_REMOVE + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    //     and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
            && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= duel::get_field_group_count(d, a.tp, 0, LOCATION_GRAVE);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DECKDES,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DECKDES, None, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.CallCoin(tp) then
    if duel::call_coin(d, a.tp) {
        // Duel.Remove(Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil),POS_FACEUP,REASON_EFFECT)
        let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    } else {
        // Duel.DiscardDeck(tp,Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE),REASON_EFFECT)
        let n = duel::get_field_group_count(d, a.tp, 0, LOCATION_GRAVE);
        duel::discard_deck(d, a.tp, n as u16, REASON_EFFECT);
    }
}
