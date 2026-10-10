//! Spell Economics
use super::prelude::*;

pub const ID: u32 = 4259068;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cost Change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_LPCOST_CHANGE)
    effect::set_code(d, e2, EFFECT_LPCOST_CHANGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.costchange)
    effect::set_value_fn(d, e2, lua_fn(costchange));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costchange(e,re,rp,val)`
fn costchange(d: &mut Duel, a: Args) -> i64 {
    let val = a.int(3);
    // if re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsSpell() and not mustpay then
    if let Some(re) = a.v(1).as_effect() {
        let h = effect::get_handler(d, re).nil();
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && card::is_spell(d, h) && !lua_global(d, "mustpay").truthy() {
            return 0;
        }
    }
    // return val
    val
}
