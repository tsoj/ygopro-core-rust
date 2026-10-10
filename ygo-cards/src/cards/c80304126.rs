//! Magician's Valkyria
use super::prelude::*;

pub const ID: u32 = 80304126;

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
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.tg)
    effect::set_value_fn(d, e1, effect_card_fn(tg));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tg(e,c)
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and c:IsFaceup() and c:IsRace(RACE_SPELLCASTER)
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)
}
