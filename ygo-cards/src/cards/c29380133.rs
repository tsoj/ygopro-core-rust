//! Yado Karu
use super::prelude::*;

pub const ID: u32 = 29380133;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // todeck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousPosition(POS_ATTACK) and e:GetHandler():IsPosition(POS_DEFENSE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, h, POS_ATTACK) && card::is_position(d, h, POS_DEFENSE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,99,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_HAND, 0, 1, 99, false, NO_EX).nil();
    // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
}
