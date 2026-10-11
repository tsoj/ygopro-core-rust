//! Dragon Horn Hunter
//! 竜角の狩猟者
use super::prelude::*;

pub const ID: u32 = 21970285;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.efilter)
    effect::set_target(d, e2, Some(effect_card_fn(efilter)));
    // e2:SetValue(200)
    effect::set_value(d, e2, 200);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // avoid battle damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.efilter)
    effect::set_target(d, e3, Some(effect_card_fn(efilter)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.efilter(e,c) return c:IsType(TYPE_NORMAL) end
fn efilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL)
}
