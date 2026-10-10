//! Beastking of the Swamps
use super::prelude::*;

pub const ID: u32 = 99426834;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion substitute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_FUSION_SUBSTITUTE)
    effect::set_code(d, e1, EFFECT_FUSION_SUBSTITUTE);
    // e1:SetCondition(s.subcon)
    effect::set_condition(d, e1, Some(condition(subcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.subcon(e)`: `return e:GetHandler():IsLocation(LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)`
fn subcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE)
}
