//! Prediction Princess Astromorrigan
use super::prelude::*;

pub const ID: u32 = 5010422;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(flipop)));
    card::register_effect(d, c, e1, false);
}
// function s.flipop(e,tp,eg,ep,ev,re,r,rp)
fn flipop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
// function s.desfilter(c) return c:IsDefensePos() end
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_defense_pos(d, c)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&desfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
        duel::damage(d, 1 - a.tp, ct as i64 * 500, REASON_EFFECT, false, None);
    }
}
