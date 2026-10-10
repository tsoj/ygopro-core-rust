//! Wetlands
use super::prelude::*;

pub const ID: u32 = 2084239;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    effect::set_value(d, e2, 1200);
    card::register_effect(d, c, e2, false);
}
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and lv<=2 and c:IsAttribute(ATTRIBUTE_WATER) and c:IsRace(RACE_AQUA)
    lv > 0 && lv <= 2 && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_race(d, c, RACE_AQUA)
}
