//! Aqua Chorus
use super::prelude::*;

pub const ID: u32 = 95132338;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    card::register_effect(d, c, e1, false);
    // Increase ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
}
/// `function(e,c) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,c:GetCode()),0,LOCATION_MZONE,LOCATION_MZONE,1,c) end`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    let codes = card::get_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, tc| card::is_faceup(d, tc) && card::is_code(d, tc, &codes)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
