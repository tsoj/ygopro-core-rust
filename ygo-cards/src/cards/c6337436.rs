//! Rapid-Fire Magician
use super::prelude::*;

pub const ID: u32 = 6337436;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if re:GetActiveType()==TYPE_SPELL and re:IsHasType(EFFECT_TYPE_ACTIVATE) and rp==tp then
    if effect::get_active_type(d, re) == TYPE_SPELL
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && a.rp == a.tp
    {
        // Duel.Damage(1-tp,400,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 400, REASON_EFFECT, false, None);
    }
}
