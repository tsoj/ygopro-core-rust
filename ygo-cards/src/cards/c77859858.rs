//! Destructive Draw
use super::prelude::*;

pub const ID: u32 = 77859858;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Register if the player has 0 cards before drawing
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PREDRAW);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(predraw)));
    card::register_effect(d, c, e2, false);
    // Draw 1 additional card during the Draw Phase
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EVENT_DRAW);
    effect::set_condition(d, e3, Some(condition(drwcon)));
    effect::set_target(d, e3, Some(cost_target(drwtg)));
    effect::set_operation(d, e3, Some(operation(drwop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // Lose 700 LP during your End Phase
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(lua_fn(e4cond)));
    effect::set_operation(d, e4, Some(operation(maintenanceop)));
    card::register_effect(d, c, e4, false);
    // Take 3000 Damage if this face-up card leaves the field
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e5, Some(operation(checkop)));
    card::register_effect(d, c, e5, false);
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e6, EVENT_LEAVE_FIELD);
    effect::set_label_object(d, e6, Value::Effect(e5));
    effect::set_operation(d, e6, Some(operation(damop)));
    card::register_effect(d, c, e6, false);
}
fn predraw(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.ep, LOCATION_HAND, 0) == 0 {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn drwcon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.r != REASON_RULE {
        return false;
    }
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, lo) == 1
}
fn drwtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.ep);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.ep, 1);
    true
}
fn drwop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn e4cond(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, a.p(1))
}
fn maintenanceop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    if duel::get_lp(d, a.tp) >= 700 {
        duel::pay_lp_cost(d, a.tp, 700);
    } else {
        duel::set_lp(d, a.tp, 0);
    }
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, c) || !card::is_status(d, c, STATUS_EFFECT_ENABLED) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) == 0 {
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        let c = effect::get_handler(d, a.e).nil();
        let p = card::get_previous_controler(d, c);
        duel::damage(d, p, 3000, REASON_EFFECT, false, None);
    }
}
