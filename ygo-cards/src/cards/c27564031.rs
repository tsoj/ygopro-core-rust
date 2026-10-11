//! Malefic World
use super::prelude::*;

pub const ID: u32 = 27564031;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PREDRAW);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    tp == duel::get_turn_player(d) && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, tp) > 0
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    let dt = duel::get_draw_count(d, tp);
    if dt != 0 {
        set_lua_global(d, "_replace_count", Value::Int(0));
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        effect::set_value(d, e1, 0);
        duel::register_effect(d, e1, tp);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let cnt = lua_global(d, "_replace_count").int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(cnt));
    let max = lua_global(d, "_replace_max").int();
    let h = effect::get_handler(d, a.e).nil();
    if cnt > max || !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    if g.len() >= 3 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, tp, 3, 3, false, NO_EX).nil();
        duel::confirm_cards(d, 1 - tp, sg.clone());
        duel::shuffle_deck(d, tp);
        let tg = group::random_select(d, &sg, 1 - tp, 1);
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
