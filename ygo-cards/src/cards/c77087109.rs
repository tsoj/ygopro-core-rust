//! Clock Resonator
use super::prelude::*;

pub const ID: u32 = 77087109;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
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
// function s.valcon(e,re,r,rp)
fn valcon(d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0 and e:GetHandler():IsDefensePos()
    let r = a.int(2);
    let h = effect::get_handler(d, a.e(0)).nil();
    (r & (REASON_EFFECT + REASON_BATTLE) as i64) != 0 && card::is_defense_pos(d, h)
}
