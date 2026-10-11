//! Savage Colosseum
use super::prelude::*;

pub const ID: u32 = 32391631;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // to defense
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_RECOVER);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target(d, e2, Some(cost_target(rectg)));
    effect::set_operation(d, e2, Some(operation(recop)));
    card::register_effect(d, c, e2, false);
    // must attack
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_MUST_ATTACK);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    card::register_effect(d, c, e3, false);
    // destroy
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_DESTROY);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e5, LOCATION_FZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    effect::set_target(d, e5, Some(cost_target(destg)));
    effect::set_operation(d, e5, Some(operation(desop)));
    card::register_effect(d, c, e5, false);
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let turn = duel::get_turn_player(d);
    duel::set_target_player(d, turn);
    duel::set_target_param(d, 300);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, turn, 300);
    true
}
fn recop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_relate_to_battle(d, duel::get_attacker(d).nil()) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    duel::recover(d, p, amount, REASON_EFFECT, false, None);
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::get_attack_announced_count(d, c) == 0
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let turn = duel::get_turn_player(d);
    let g = duel::get_matching_group(d, Some(&desfilter), turn, LOCATION_MZONE, 0, c.into());
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let turn = duel::get_turn_player(d);
    let g = duel::get_matching_group(d, Some(&desfilter), turn, LOCATION_MZONE, 0, c.into());
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
