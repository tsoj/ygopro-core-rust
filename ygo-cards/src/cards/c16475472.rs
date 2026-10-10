//! Lesser Fiend
use super::prelude::*;

pub const ID: u32 = 16475472;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --redirect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_BATTLE_DESTROY_REDIRECT)
    effect::set_code(d, e1, EFFECT_BATTLE_DESTROY_REDIRECT);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
