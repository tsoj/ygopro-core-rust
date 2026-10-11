//! Gearbreed
use super::prelude::*;

pub const ID: u32 = 27979109;
pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_GEMINI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PREDRAW);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // Draw
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    effect::set_target(d, e3, Some(cost_target(drtg)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, a.tp) > 0
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_EQUIP) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let dt = duel::get_draw_count(d, a.tp);
    if dt == 0 {
        return;
    }
    set_lua_global(d, "_replace_count", Value::Int(1));
    set_lua_global(d, "_replace_max", Value::Int(dt as i64));
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_DRAW_COUNT);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
    effect::set_value(d, e1, 0);
    duel::register_effect(d, e1, a.tp);
    if lua_global(d, "_replace_count").int() > lua_global(d, "_replace_max").int() || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP) && card::is_spell(d, c)
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_SZONE, 0, 1, NO_EX)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_able_to_deck_as_cost(d, c) && card::is_type(d, c, TYPE_GEMINI)) || (card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_FIRE))
}
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
