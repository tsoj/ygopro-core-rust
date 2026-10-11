//! ホープ・オブ・フィフス (Fifth Hope)
use super::prelude::*;

pub const ID: u32 = 55428811;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // local dc=2
    let mut dc: u32 = 2;
    // if not Duel.IsExistingMatchingCard(nil,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,e:GetHandler()) then dc=3 end
    let h = effect::get_handler(d, a.e).nil();
    if !duel::is_existing_matching_card(d, None, a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, h.into()) {
        dc = 3;
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,dc) and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,5,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, dc) && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 5, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,5,5,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 5, 5, false, NO_EX).nil();
    // e:SetLabel(dc)
    effect::set_label(d, a.e, &[dc as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,dc)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, dc as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not tg or tg:FilterCount(Card.IsRelateToEffect,nil,e)~=5 then return end
    let Some(tg) = tg else {
        return;
    };
    if group::filter_count(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX) != 5 {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==5 then
    if ct == 5 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,e:GetLabel(),REASON_EFFECT)
        let label = effect::get_label(d, a.e) as u32;
        duel::draw(d, a.tp, label, REASON_EFFECT);
    }
}
