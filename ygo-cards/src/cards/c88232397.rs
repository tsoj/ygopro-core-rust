//! Skilled Brown Magician
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 88232397;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_SPELL],
    listed_names: &[40703222],
    listed_series: &[SET_KURIBOH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 3);
    // Place Spell Counter
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_MZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(acop)));
    card::register_effect(d, c, e1, false);
    // Change Level or add to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
}
fn acop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, c, 1) > 0 {
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 1, REASON_COST);
    }
    card::remove_counter(d, c, a.tp, COUNTER_SPELL as u16, 1, REASON_COST);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_code(d, c, &[40703222]) || (card::is_set_card(d, c, &[SET_KURIBOH as u16]) && card::is_monster(d, c))) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let b2 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    let op = duel::select_effect(d, tp, &[(true, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE + CATEGORY_LVCHANGE);
    } else {
        effect::set_category(d, a.e, CATEGORY_SEARCH + CATEGORY_TOHAND);
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 0, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if effect::get_label(d, a.e) == 1 {
        let c = effect::get_handler(d, a.e).nil();
        if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
            return;
        }
        // Increase Level
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
        // Increase ATK
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e2, 1500);
        card::register_effect(d, c, e2, false);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let f = aux::necro_valley_filter(|d, c| thfilter(d, c));
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&f),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
