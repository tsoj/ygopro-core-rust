//! Lancephorhynchus
use super::prelude::*;

pub const ID: u32 = 48940337;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.target(e,c)
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL)
    card::is_type(d, c, TYPE_NORMAL)
}
