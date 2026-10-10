//! Flame Tiger
use super::prelude::*;

pub const ID: u32 = 66499018;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PREDRAW);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    tp == duel::get_turn_player(d)
        && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_draw_count(d, tp) > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    let dt = duel::get_draw_count(d, tp);
    if dt != 0 {
        // (Lua globals shared with the other draw-replacing scripts)
        set_lua_global(d, "_replace_count", Value::Int(0));
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        effect::set_value(d, e1, 0);
        duel::register_effect(d, e1, tp);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn global_int(d: &Duel, name: &str) -> i64 {
    match lua_global(d, name) {
        Value::Int(x) => x,
        _ => crate::lua_error("attempt to perform arithmetic on a nil value"),
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let count = global_int(d, "_replace_count") + 1;
    set_lua_global(d, "_replace_count", Value::Int(count));
    if count <= global_int(d, "_replace_max")
        && card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
    {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
