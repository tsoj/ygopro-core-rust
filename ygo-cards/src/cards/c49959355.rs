//! Uni-Zombie
use super::prelude::*;

pub const ID: u32 = 49959355;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // level
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(lvcost)));
    effect::set_target(d, e1, Some(cost_target(lvtg1)));
    effect::set_operation(d, e1, Some(operation(lvop1)));
    card::register_effect(d, c, e1, false);
    // level
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(lvcost)));
    effect::set_target(d, e2, Some(cost_target(lvtg2)));
    effect::set_operation(d, e2, Some(operation(lvop2)));
    card::register_effect(d, c, e2, false);
}
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::has_level(d, c) && card::get_level(d, c) != 0
}
fn lvtg1(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn lvop1(d: &mut Duel, a: &EffArgs) {
    if duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) == 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_grave(d, c, None)
}
fn lvtg2(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn lvop2(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::send_to_grave(d, &g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e3, 1, 0);
    duel::register_effect(d, e3, a.tp);
}
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_ZOMBIE)
}
