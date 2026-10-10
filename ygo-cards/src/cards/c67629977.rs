//! Hoshiningen
use super::prelude::*;

pub const ID: u32 = 67629977;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_target(d, e1, Some(effect_card_fn(tg1)));
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_target(d, e2, Some(effect_card_fn(tg2)));
    effect::set_value(d, e2, -400);
    card::register_effect(d, c, e2, false);
}
// function s.tg1(e,c)
fn tg1(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
// function s.tg2(e,c)
fn tg2(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
}
