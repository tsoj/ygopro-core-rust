//! Versago the Destroyer
use super::prelude::*;

pub const ID: u32 = 50259460;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion substitute
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_FUSION_SUBSTITUTE);
    effect::set_condition(d, e1, Some(condition(subcon)));
    card::register_effect(d, c, e1, false);
}
fn subcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE)
}
