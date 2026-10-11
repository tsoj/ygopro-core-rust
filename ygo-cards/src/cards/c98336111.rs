//! Fusion Devourer
use super::prelude::*;

pub const ID: u32 = 98336111;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condtion(e)`
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()~=nil
    //     and (Duel.GetAttacker()==e:GetHandler() or Duel.GetAttackTarget()==e:GetHandler())
    if !((ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h) || duel::get_attack_target(d) == Some(h)
}
/// `s.target(e,c)`
fn target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetBattleTarget() and c:IsType(TYPE_FUSION)
    let h = effect::get_handler(d, e).nil();
    card::get_battle_target(d, h) == Some(c) && card::is_type(d, c, TYPE_FUSION)
}
