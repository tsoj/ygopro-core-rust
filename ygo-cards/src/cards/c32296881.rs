//! Bountiful Artemis
use super::prelude::*;

pub const ID: u32 = 32296881;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsActiveType(TYPE_COUNTER) then return end
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || !effect::is_active_type(d, re, TYPE_COUNTER) {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
