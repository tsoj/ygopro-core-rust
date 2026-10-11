//! Invincibility Barrier
use super::prelude::*;

pub const ID: u32 = 17722185;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Activate
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EXTRA + TYPE_RITUAL + TYPE_PENDULUM)
        && card::is_able_to_remove_as_cost(d, c, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    let ty = card::get_type(d, tc) & (TYPE_EXTRA + TYPE_RITUAL + TYPE_PENDULUM);
    effect::set_label(d, a.e, &[ty as i64]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let ct = effect::get_label(d, a.e);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    effect::set_label(d, e1, &[ct]);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let label = effect::get_label(d, e);
    card::is_type(d, c, label as u32)
}
