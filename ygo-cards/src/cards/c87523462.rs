//! Winged Sage Falcos
use super::prelude::*;

pub const ID: u32 = 87523462;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place destroyed monster on top of the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_condition(d, e1, Some(condition(tdcon)));
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if !card::is_relate_to_battle(d, c) || card::is_facedown(d, c) {
        return false;
    }
    let bc = bc.nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc) && card::is_controler(d, bc, 1 - a.tp) && card::is_previous_position(d, bc, POS_FACEUP_ATTACK)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, bc);
    }
    duel::set_target_card(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
