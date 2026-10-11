//! Dark Deal
use super::prelude::*;

pub const ID: u32 = 65824822;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change effect of Normal Spell to "Your opponent discards 1 random card"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    a.ep == 1 - a.tp && card::is_normal_spell(d, rc) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let g = CardSet::new();
    duel::change_target_card(d, a.ev as u8, &g);
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::cancel_to_grave(d, h, false);
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    if !g.is_empty() {
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
