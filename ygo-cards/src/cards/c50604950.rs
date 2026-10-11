//! X-Saber Galahad
use super::prelude::*;

pub const ID: u32 = 50604950;

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
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // if ph~=PHASE_DAMAGE and ph~=PHASE_DAMAGE_CAL then return 0 end
    if ph != PHASE_DAMAGE && ph != PHASE_DAMAGE_CAL {
        return 0;
    }
    // if c==Duel.GetAttacker() and Duel.GetAttackTarget() then return 300 end
    if duel::get_attacker(d) == Some(c) && duel::get_attack_target(d).is_some() {
        return 300;
    }
    // if c==Duel.GetAttackTarget() then return -500 end
    if duel::get_attack_target(d) == Some(c) {
        return -500;
    }
    // return 0
    0
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,e:GetHandler(),SET_SABER) end
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, cc| card::is_set_card(d, cc, &[SET_SABER as u16])), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,e:GetHandler(),SET_SABER)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, cc| card::is_set_card(d, cc, &[SET_SABER as u16])), 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    // Duel.NegateAttack()
    duel::negate_attack(d);
}
