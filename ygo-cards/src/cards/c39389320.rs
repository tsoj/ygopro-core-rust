//! Battleguard King
use super::prelude::*;

pub const ID: u32 = 39389320;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // multi attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| card::is_race(d, x, RACE_WARRIOR)), 1, 1, false, None, c.into());
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, x| card::is_race(d, x, RACE_WARRIOR)), 1, 10, false, None, c.into());
    let ct = duel::release(d, g, REASON_COST, None);
    effect::set_label(d, e, &[ct as i64]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        effect::set_value(d, e1, effect::get_label(d, a.e));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
}
