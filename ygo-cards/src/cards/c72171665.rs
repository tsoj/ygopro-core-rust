//! Magical Broker
use super::prelude::*;

pub const ID: u32 = 72171665;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle or card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e1, Some(lua_fn(incon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e2, false);
    // Send 1 face-up Continuous Spell/Trap to the GY to draw 1 card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    effect::set_target(d, e3, Some(cost_target(drtg)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
}
/// `s.indesfil(c)`
fn indesfil(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_CONTINUOUS) && card::is_spell_trap(d, c)
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&indesfil), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.drcon(e,tp,...)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = d;
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_CONTINUOUS) && card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.drcost(e,tp,...,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
