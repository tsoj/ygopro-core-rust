//! Generaider Boss Quest
//! 王の試練
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 75448086;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GENERAIDER) and c:IsMonster() and not c:IsPublic() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_GENERAIDER as u16]) && card::is_monster(d, c) && !card::is_public(d, c) && card::is_able_to_deck(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsCode(id) and c:IsSetCard(SET_GENERAIDER) and c:IsSpellTrap() and c:IsAbleToHand()
    !card::is_code(d, c, &[ID]) && card::is_set_card(d, c, &[SET_GENERAIDER as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil)
    //     and aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let rc = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #rc>0 then
    if !rc.is_empty() {
        // Duel.ConfirmCards(1-tp,rc)
        duel::confirm_cards(d, 1 - a.tp, rc.clone());
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
        // if #sg>0 and Duel.SendtoHand(sg,nil,REASON_EFFECT)~=0 then
        if !sg.is_empty() && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) != 0 {
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, a.tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(rc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, rc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
}
