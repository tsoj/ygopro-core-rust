//! Abyssal Kingshark
use super::prelude::*;

pub const ID: u32 = 44223284;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(s.valcon)
    effect::set_value_fn(d, e1, lua_fn(valcon));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.valcon(e,re,r,rp)`
fn valcon(d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0 and not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    (a.int(2) as u32 & REASON_EFFECT) != 0 && !effect::is_has_property(d, a.e(1), EFFECT_FLAG_CARD_TARGET, 0)
}
