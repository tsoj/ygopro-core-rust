//! Emissary of the Afterlife
use super::prelude::*;

pub const ID: u32 = 75043725;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsType(TYPE_NORMAL) and c:IsAbleToHand()
    card::is_level_below(d, c, 3) && card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_hand(d, c, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let mut g1 = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_ATOHAND as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,s.filter,tp,0,LOCATION_DECK,1,1,nil)
    let g2 = duel::select_matching_card(d, 1 - a.tp, Some(&filter), a.tp, 0, LOCATION_DECK, 1, 1, false, NO_EX).nil();
    // local tc2=g2:GetFirst()
    let tc2 = group::get_first(d, &g2);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoHand(g1,nil,REASON_EFFECT)
    duel::send_to_hand(d, g1, None, REASON_EFFECT, None);
    // if tc1 then Duel.ConfirmCards(1-tp,tc1) end
    if let Some(tc1) = tc1 {
        duel::confirm_cards(d, 1 - a.tp, tc1);
    }
    // if tc2 then Duel.ConfirmCards(tp,tc2) end
    if let Some(tc2) = tc2 {
        duel::confirm_cards(d, a.tp, tc2);
    }
}
