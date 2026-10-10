//! Remove Brainwashing
use super::prelude::*;

pub const ID: u32 = 94739788;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_BRAINWASHING_CHECK)
    duel::enable_global_flag(d, GLOBALFLAG_BRAINWASHING_CHECK);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // e2
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_REMOVE_BRAINWASHING);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    card::register_effect(d, c, e2, false);
}
