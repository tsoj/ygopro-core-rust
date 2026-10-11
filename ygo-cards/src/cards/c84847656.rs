//! Telekinetic Shocker
use super::prelude::*;

pub const ID: u32 = 84847656;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy replace
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_value_fn(d, e1, effect_card_fn(|d, _e, c| dfilter(d, c)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC) && !card::is_reason(d, c, REASON_REPLACE)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let h = effect::get_handler(d, a.e);
        let contains = h.is_some_and(|h| group::is_contains(&eg, h));
        return !contains && duel::check_lp_cost(d, a.tp, 500) && group::is_exists(d, &eg, &|d, c| dfilter(d, c), 1, NO_EX);
    }
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, h, Some(96)) {
        duel::pay_lp_cost(d, a.tp, 500);
        true
    } else {
        false
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, CardSet::from_card(c), REASON_EFFECT | REASON_REPLACE, None, None);
}
