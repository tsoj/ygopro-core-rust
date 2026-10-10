//! Non-Fusion Area
use super::prelude::*;

pub const ID: u32 = 27581098;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // disable spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    let sumtp = a.int(3);
    (sumtp & SUMMON_TYPE_FUSION as i64) == SUMMON_TYPE_FUSION as i64
}
