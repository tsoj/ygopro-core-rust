//! Vessel for the Dragon Cycle
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 65124425;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TENYI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 Wyrm monster from your Deck to the GY, then, if you control a face-up non-Effect Monster, you can add from your Deck to your hand 1 "Tenyi" monster with a different name from the monster sent to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_TOHAND | CATEGORY_SEARCH);
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
/// `s.gyfilter(c)`: `return c:IsRace(RACE_WYRM) and c:IsAbleToGrave()`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_WYRM) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c,code)`: `return c:IsSetCard(SET_TENYI) and c:IsMonster() and not c:IsCode(code) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_set_card(d, c, &[SET_TENYI as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local gc=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &sel);
    // if not gc or Duel.SendtoGrave(gc,REASON_EFFECT)==0 or not gc:IsLocation(LOCATION_GRAVE) then return end
    let Some(gc) = gc else { return };
    if duel::send_to_grave(d, gc, REASON_EFFECT, None, None) == 0 || !card::is_location(d, gc, LOCATION_GRAVE) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil,gc:GetCode())
    let code = card::get_code(d, gc)[0];
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, code)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsNonEffectMonster),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_non_effect_monster(d, c))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
