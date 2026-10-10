//! Disposable Learner Device
use super::prelude::*;

pub const ID: u32 = 42149850;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Equipped monster gains 200 ATK per monster in GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // Register the fact it was sent from field to GY
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(regcon)));
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
    // Set itself from GY
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e3, Some(condition(setcon)));
    effect::set_target(d, e3, Some(cost_target(settg)));
    effect::set_operation(d, e3, Some(operation(setop)));
    card::register_effect(d, c, e3, false);
}
fn val(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    duel::get_matching_group_count(d, Some(&card::is_monster), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 200
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, h, false, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        duel::s_set(d, a.tp, c, None, true);
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, false);
    }
}
