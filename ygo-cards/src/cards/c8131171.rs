//! Sinister Serpent
use super::prelude::*;

pub const ID: u32 = 8131171;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add this card to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
    // Banish 1 "Sinister Serpent" from your GY during the opponent's next End Phase
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(rmcon)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    duel::register_effect(d, e1, a.tp);
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
