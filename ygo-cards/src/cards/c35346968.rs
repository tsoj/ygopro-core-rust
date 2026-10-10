//! Solemn Wishes
use super::prelude::*;

pub const ID: u32 = 35346968;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // recover
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_DRAW);
    effect::set_operation(d, e2, Some(operation(recop)));
    card::register_effect(d, c, e2, false);
}
fn recop(d: &mut Duel, a: &EffArgs) {
    if a.ep != a.tp {
        return;
    }
    duel::recover(d, a.tp, 500, REASON_EFFECT, false, None);
}
