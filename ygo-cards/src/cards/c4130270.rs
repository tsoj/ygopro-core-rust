//! G.B. Hunter
//! G・B・ハンター
use super::prelude::*;

pub const ID: u32 = 4130270;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_TO_DECK)
    effect::set_code(d, e1, EFFECT_CANNOT_TO_DECK);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e2, EFFECT_CANNOT_TO_HAND);
    // e2:SetTarget(s.etarget)
    effect::set_target(d, e2, Some(effect_card_fn(etarget)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetOriginalType()&TYPE_EXTRA~=0
    card::get_original_type(d, c) & TYPE_EXTRA != 0
}
