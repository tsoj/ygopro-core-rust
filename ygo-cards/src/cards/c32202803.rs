//! Bujingi Wolf
//! 武神器－タルタ
use super::prelude::*;

pub const ID: u32 = 32202803;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
    c != effect::get_handler(d, e).nil() && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
}
