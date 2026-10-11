//! Aqua Chorus Round
use super::prelude::*;

pub const ID: u32 = 59131526;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 card from the hand and search 2 with the same name during your next Standby Phase
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn removable(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&removable), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    let n = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    effect::set_label(d, a.e, &[n as i64]);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_HAND as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&removable), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = match group::get_first(d, &g) {
        Some(tc) => tc,
        None => return,
    };
    if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Add up to 2 cards during the Standby Phase
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    let mut labels = vec![duel::get_turn_count(d, None) as i64];
    labels.extend(card::get_code(d, tc).into_iter().map(|x| x as i64));
    effect::set_label(d, e1, &labels);
    effect::set_condition(d, e1, Some(lua_fn(thcond)));
    effect::set_operation(d, e1, Some(operation(thop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
    duel::register_effect(d, e1, a.tp);
    if effect::get_label(d, a.e) == 0 {
        // If the opponent had 0 cards when this was activated, you cannot activate cards with the same name
        let e2 = effect::create_effect(d, h);
        effect::set_description(d, e2, aux::stringid(ID, 1));
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
        effect::set_target_range(d, e2, 1, 0);
        effect::set_value_fn(d, e2, lua_fn(aclimit));
        effect::set_label_object(d, e2, Value::Card(tc));
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e2, a.tp);
    }
}
/// `function(e,re,tp) return re:GetHandler():IsOriginalCode(tc:GetCode()) end`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    let tc = effect::get_label_object(d, e).as_card().nil();
    let codes = card::get_code(d, tc);
    card::is_original_code(d, h, &codes)
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel() end`
fn thcond(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    duel::is_turn_player(d, a.p(1)) && duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
}
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let labels = effect::get_labels(d, a.e);
    let code = labels[1] as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, code)), a.tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2) as u64) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, a.tp, 1, 2, false, NO_EX).nil();
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
