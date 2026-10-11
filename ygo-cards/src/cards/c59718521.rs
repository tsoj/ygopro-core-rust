//! Mind Over Matter
use super::prelude::*;

pub const ID: u32 = 59718521;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target1)));
    effect::set_operation(d, e1, Some(operation(activate1)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON);
    card::register_effect(d, c, e2, false);
    // Activate(effect)
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_condition(d, e3, Some(condition(cond2)));
    effect::set_cost(d, e3, Some(cost_target(cost_fn)));
    effect::set_target(d, e3, Some(cost_target(target2)));
    effect::set_operation(d, e3, Some(operation(activate2)));
    card::register_effect(d, c, e3, false);
}
fn cond1(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_chain(d, true) == 0
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_PSYCHIC) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&filter), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&filter), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let n = a.eg.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), n, 0, 0);
    true
}
fn activate1(d: &mut Duel, a: &EffArgs) {
    duel::negate_summon(d, a.eg());
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate2(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
