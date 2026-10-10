//! Hieracosphinx
use super::prelude::*;

pub const ID: u32 = 82260502;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // at limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, effect_card_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE)
    card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
