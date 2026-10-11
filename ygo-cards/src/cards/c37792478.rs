//! Metabo-Shark
//! メタボ・シャーク
use super::prelude::*;

pub const ID: u32 = 37792478;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is Normal Summoned: You can target 2 Fish monsters in your GY; shuffle those targets into your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsAbleToDeck()
    card::is_race(d, c, RACE_FISH) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == a.tp && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 2, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e):Match(Card.IsRace,nil,RACE_FISH)
    let targets = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &targets, &|d, c| card::is_race(d, c, RACE_FISH), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
