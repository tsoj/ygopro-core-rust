//! Morphtronic Videon
use super::prelude::*;

pub const ID: u32 = 84592800;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.cona)
    effect::set_condition(d, e1, Some(lua_fn(cona)));
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // def
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetCondition(s.cond)
    effect::set_condition(d, e2, Some(lua_fn(cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cona(e)`
fn cona(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, h)
}
/// `s.cond(e)`
fn cond(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, h)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetEquipCount()*800
    card::get_equip_count(d, c) as i64 * 800
}
