//! Appropriate
use super::prelude::*;

pub const ID: u32 = 48539234;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_DRAW);
    effect::set_condition(d, e1, Some(condition(cond)));
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_DRAW);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.GetCurrentPhase()~=PHASE_DRAW
    a.ep != a.tp && duel::get_current_phase(d) != PHASE_DRAW
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Draw(tp,2,REASON_EFFECT)
    duel::draw(d, a.tp, 2, REASON_EFFECT);
}
