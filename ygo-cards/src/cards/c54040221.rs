//! Royal Firestorm Guards
use super::prelude::*;

pub const ID: u32 = 54040221;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to deck and draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PYRO) and c:IsAbleToDeck()
    card::is_race(d, c, RACE_PYRO) && card::is_able_to_deck(d, c)
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_controler(d, chkc, a.tp)
            && filter(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,4,nil) then
    if duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 4, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,4,4,nil)
        let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 4, 4, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let tg = info[0].as_group().cloned();
    // if not tg or tg:FilterCount(Card.IsRelateToEffect,nil,e)~=4 then return end
    let tg = match tg {
        Some(g) => g,
        None => return,
    };
    if group::filter_count(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX) != 4 {
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
    // if ct==4 then
    if ct == 4 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
}
