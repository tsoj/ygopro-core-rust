//! The Agent of Mystery - Earth
use super::prelude::*;

pub const ID: u32 = 91188343;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_SANCTUARY_SKY, 55794644], listed_series: &[SET_THE_AGENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "The Agent" monster from your Deck to your hand, except "The Agent of Mystery - Earth", or if "The Sanctuary in the Sky" is on the field, you can add 1 "Master Hyperion" instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thilter(c,sanct_chk)`
fn thilter(d: &mut Duel, c: CardId, sanct_chk: bool) -> bool {
    // return ((c:IsSetCard(SET_THE_AGENT) and c:IsMonster() and not c:IsCode(id)) or (sanct_chk and c:IsCode(55794644))) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_THE_AGENT as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]))
        || (sanct_chk && card::is_code(d, c, &[55794644])))
        && card::is_able_to_hand(d, c, None)
}
/// `local sanct_chk=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) or Duel.IsEnvironment(CARD_SANCTUARY_SKY)`
fn sanct_check(d: &mut Duel, tp: u8) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_SANCTUARY_SKY])), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        || duel::is_environment(d, CARD_SANCTUARY_SKY, None, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local sanct_chk=...
        let sanct_chk = sanct_check(d, a.tp);
        // return Duel.IsExistingMatchingCard(s.thilter,tp,LOCATION_DECK,0,1,nil,sanct_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| thilter(d, c, sanct_chk)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local sanct_chk=...
    let sanct_chk = sanct_check(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thilter,tp,LOCATION_DECK,0,1,1,nil,sanct_chk)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thilter(d, c, sanct_chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
