//! Raidraptor - Blaze Falcon
use super::prelude::*;

pub const ID: u32 = 45533023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WINGEDBEAST, sc, st, Some(tp)))), Some(5), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // direct attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e1, Some(condition(dacon)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg1)));
    effect::set_operation(d, e2, Some(operation(desop1)));
    card::register_effect(d, c, e2, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e3, Some(cost_target(destg2)));
    effect::set_operation(d, e3, Some(operation(desop2)));
    card::register_effect(d, c, e3, false);
}
fn dacon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, c) > 0
}
fn descon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop1(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_special_summoned(d, c)
}
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, a.tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (n * 500) as i32);
    true
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let sg = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_MZONE, NO_EX);
    let ct = duel::destroy(d, sg, REASON_EFFECT, None, None);
    if ct > 0 {
        duel::damage(d, 1 - tp, (ct as i64) * 500, REASON_EFFECT, false, None);
    }
}
