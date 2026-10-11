//! Batteryman D
use super::prelude::*;

pub const ID: u32 = 55401221;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE as u32);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, lua_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atlimit(e,c)`: `return c:IsFaceup() and c:IsRace(RACE_THUNDER) and c:GetCode()~=id`
fn atlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    card::is_faceup(d, c) && card::is_race(d, c, RACE_THUNDER as u64) && card::get_code(d, c)[0] != ID
}
