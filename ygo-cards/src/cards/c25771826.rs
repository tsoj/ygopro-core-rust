//! Ally of Justice Garadholg
use super::prelude::*;

pub const ID: u32 = 25771826;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetValue(200)
    effect::set_value(d, e1, 200);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // if not (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) then return false end
    if !(ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    let h = effect::get_handler(d, a.e).nil();
    // return (a==e:GetHandler() and d and d:IsFaceup() and d:IsAttribute(ATTRIBUTE_LIGHT))
    //   or (d==e:GetHandler() and a:IsAttribute(ATTRIBUTE_LIGHT))
    if at == Some(h) {
        if let Some(dt) = dt {
            if card::is_faceup(d, dt) && card::is_attribute(d, dt, ATTRIBUTE_LIGHT) {
                return true;
            }
        }
    }
    dt == Some(h) && {
        let at = duel::get_attacker(d).nil();
        card::is_attribute(d, at, ATTRIBUTE_LIGHT)
    }
}
