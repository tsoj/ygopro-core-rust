//! Phoenix Beast Gairuda
use super::prelude::*;

pub const ID: u32 = 43791861;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condtion(e)
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL)
    //   and Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()~=nil
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL)
        && duel::get_attacker(d) == Some(effect::get_handler(d, a.e).nil())
        && duel::get_attack_target(d).is_some()
}
