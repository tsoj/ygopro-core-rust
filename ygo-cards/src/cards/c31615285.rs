//! Cactus Bouncer
use super::prelude::*;

pub const ID: u32 = 31615285;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // disable spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_condition(d, e2, Some(condition(dscon)));
    card::register_effect(d, c, e2, false);
}
fn dscon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(
        d,
        Some(&|d, c2| card::is_faceup(d, c2) && card::is_race(d, c2, RACE_PLANT)),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        c.into(),
    )
}
