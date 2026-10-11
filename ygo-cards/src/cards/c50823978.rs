//! Piranha Army
//! 軍隊ピラニア
use super::prelude::*;

pub const ID: u32 = 50823978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // deepen damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetCondition(s.dcon)
    effect::set_condition(d, e1, Some(condition(dcon)));
    // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.dcon(e)`: `return Duel.GetAttackTarget()==nil`
fn dcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_attack_target(d).is_none()
}
