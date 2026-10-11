//! Seal of Wickedness
use super::prelude::*;

pub const ID: u32 = 17874674;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DRAW_PHASE));
    effect::set_target(d, e1, Some(cost_target(target1)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // confirm
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target2)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // cancel target
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_TURN_END);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_operation(d, e3, Some(operation(ctarget)));
    card::register_effect(d, c, e3, false);
    // cost
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(costcon)));
    effect::set_operation(d, e4, Some(operation(costop)));
    card::register_effect(d, c, e4, false);
}
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    let e = a.e;
    let tp = a.tp;
    if duel::is_turn_player(d, 1 - tp)
        && duel::is_phase(d, PHASE_STANDBY)
        && duel::is_existing_target(d, Some(&card::is_negatable), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, tp, 94)
    {
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        duel::select_target(d, tp, Some(&card::is_negatable), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        let c = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    } else {
        effect::set_property(d, e, 0, 0);
    }
    true
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&card::is_negatable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, c, ID) == 0 || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let Some(tc) = duel::first_target(d) else { return };
    if ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER))
        && card::is_relate_to_effect(d, tc, a.e)
    {
        card::set_card_target(d, c, tc);
        effect::set_label_object(d, a.e, Value::Card(tc));
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_condition(d, e1, Some(condition(rcon)));
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE, 0);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        effect::set_condition(d, e2, Some(condition(rcon)));
        card::register_effect(d, tc, e2, false);
    }
}
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h)
}
fn ctarget(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_first_card_target(d, c) {
        card::cancel_card_target(d, c, tc);
    }
}
fn costcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn costop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::check_lp_cost(d, tp, 500) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        duel::pay_lp_cost(d, tp, 500);
    } else {
        let c = effect::get_handler(d, a.e).nil();
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
