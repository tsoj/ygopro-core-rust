//! Live☆Twin Channel
use super::prelude::*;

pub const ID: u32 = 35487920;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KI_SIKIL, SET_LIL_LA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Negate an attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
    // Return 1 "Ki-sikil" or "Lil-la" monster from your GY to your Deck or hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let setcodes = |d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_KI_SIKIL as u16, SET_LIL_LA as u16]);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&setcodes), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&setcodes), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn negop(d: &mut Duel, _a: &EffArgs) {
    duel::negate_attack(d);
}
fn tgfilter(d: &mut Duel, c: CardId, mon_check: bool) -> bool {
    card::is_set_card(d, c, &[SET_KI_SIKIL as u16, SET_LIL_LA as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_deck(d, c) || (mon_check && card::is_able_to_hand(d, c, None)))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let mon_check = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, mon_check);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, mon_check)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, mon_check)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    if !mon_check {
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, tp, 0);
    } else {
        duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, tp, 0);
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
        && card::is_able_to_hand(d, tc, None)
        && (!card::is_able_to_deck(d, tc) || duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)))
    {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, tc);
    } else {
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
