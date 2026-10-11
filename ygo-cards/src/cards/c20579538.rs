//! Sylvan Princessprout
use super::prelude::*;

pub const ID: u32 = 20579538;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPROUT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate and send to the GY and return card to the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Special Summon itself and change level
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPROUT as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
fn op(d: &mut Duel, a: &EffArgs) {
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    duel::confirm_decktop(d, a.tp, 1);
    let g = duel::get_decktop_group(d, a.tp, 1);
    duel::disable_shuffle_check(d, true);
    if duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None) == 0 {
        return;
    }
    let dg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(tdfilter)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let tg = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, tg.clone(), true);
        duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    let lv = duel::announce_number_range(d, a.tp, Some(1), Some(8), &[]).nil().0;
    duel::set_target_param(d, lv as u32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e1, lv);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
