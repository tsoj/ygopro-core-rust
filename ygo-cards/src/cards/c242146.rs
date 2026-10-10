//! Ballista of Rampart Smashing
use super::prelude::*;

pub const ID: u32 = 242146;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(1500)
    effect::set_value(d, e2, 1500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetCurrentPhase()~=PHASE_DAMAGE_CAL then return false end
    if duel::get_current_phase(d) != PHASE_DAMAGE_CAL {
        return false;
    }
    // local eqc=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, c);
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()  (Lua's local d is `dt` here: `d` is the duel)
    let dt = duel::get_attack_target(d);
    // return d and a==eqc and d:GetBattlePosition()==POS_FACEDOWN_DEFENSE
    match dt {
        Some(dt) => att == eqc && card::get_battle_position(d, dt) == POS_FACEDOWN_DEFENSE as u32,
        None => false,
    }
}
