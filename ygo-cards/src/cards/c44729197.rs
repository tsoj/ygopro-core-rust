//! Steamroid
use super::prelude::*;

pub const ID: u32 = 44729197;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condtion(e)`
fn condtion(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL
    ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL
}
/// `s.val(e,c)`
fn val(d: &mut Duel, e: EffectId) -> i64 {
    // if Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()~=nil then return 500
    if duel::get_attacker(d) == effect::get_handler(d, e) && duel::get_attack_target(d).is_some() {
        500
    // elseif e:GetHandler()==Duel.GetAttackTarget() then return -500
    } else if effect::get_handler(d, e) == duel::get_attack_target(d) {
        -500
    // else return 0 end
    } else {
        0
    }
}
