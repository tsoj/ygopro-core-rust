//! Zexal Construction
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 62623659;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZEXAL, SET_ZW, SET_ZS, SET_RANK_UP_MAGIC, SET_RANK_DOWN_MAGIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_TODECK);
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
/// `s.filter(c)`: `return c:IsAbleToDeck() and not c:IsPublic()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_deck(d, c) && !card::is_public(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and ((c:IsMonster() and (c:IsSetCard(SET_ZW) or c:IsSetCard(SET_ZS)))
    //   or (c:IsSpell() and (c:IsSetCard(SET_ZEXAL) or c:IsSetCard(SET_RANK_UP_MAGIC) or c:IsSetCard(SET_RANK_DOWN_MAGIC)))
    //   or c:IsTrap() and c:IsSetCard(SET_ZEXAL))
    card::is_able_to_hand(d, c, None)
        && ((card::is_monster(d, c)
            && (card::is_set_card(d, c, &[SET_ZW as u16]) || card::is_set_card(d, c, &[SET_ZS as u16])))
            || (card::is_spell(d, c)
                && (card::is_set_card(d, c, &[SET_ZEXAL as u16])
                    || card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16])
                    || card::is_set_card(d, c, &[SET_RANK_DOWN_MAGIC as u16])))
            || (card::is_trap(d, c) && card::is_set_card(d, c, &[SET_ZEXAL as u16])))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,e:GetHandler())
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_HAND, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil)
    let rg = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if rg:GetFirst() then
    if group::get_first(d, &rg).is_some() {
        // Duel.ConfirmCards(1-tp,rg)
        duel::confirm_cards(d, 1 - tp, rg.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let tg = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if tg:GetFirst() then
        if group::get_first(d, &tg).is_some() {
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - tp, tg);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(rg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, rg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
