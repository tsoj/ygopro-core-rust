//! Cyber Ogre 2
use super::prelude::*;

pub const ID: u32 = 37057012;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CYBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,64268668,2)
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::Code(64268668), 2)]);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL)
    //     and e:GetHandler()==Duel.GetAttacker() and Duel.GetAttackTarget()~=nil
    duel::is_phase(d, PHASE_DAMAGE_CAL) && {
        let h = effect::get_handler(d, a.e).nil();
        Some(h) == duel::get_attacker(d) && duel::get_attack_target(d).is_some()
    }
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetAttackTarget():GetAttack()/2
    let t = duel::get_attack_target(d).nil();
    let atk = card::get_attack(d, t);
    lua_int(atk as f64 / 2.0)
}
