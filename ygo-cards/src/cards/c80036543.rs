//! Hope for Escape
use super::prelude::*;

pub const ID: u32 = 80036543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) <= duel::get_lp(d, 1 - a.tp) - 1000
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let p1 = duel::get_lp(d, a.tp);
    let p2 = duel::get_lp(d, 1 - a.tp);
    let mut s = p2 - p1;
    if s < 0 {
        s = p1 - p2;
    }
    // math.floor(s/2000)
    let dr = s.div_euclid(2000);
    duel::draw(d, a.tp, dr as u32, REASON_EFFECT);
}
