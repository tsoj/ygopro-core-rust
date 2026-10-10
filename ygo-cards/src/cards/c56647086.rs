//! Invader of Darkness
use super::prelude::*;

pub const ID: u32 = 56647086;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // act limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsQuickPlaySpell() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    let c = effect::get_handler(d, re).nil();
    card::is_quick_play_spell(d, c) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
