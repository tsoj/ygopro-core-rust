//! Uminotaurus
use super::prelude::*;

pub const ID: u32 = 30464153;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,c)
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA)
    card::is_race(d, c, (RACE_FISH | RACE_SEASERPENT | RACE_AQUA) as u64)
}
