//! Gravekeeper's Vassal
use super::prelude::*;

pub const ID: u32 = 99690140;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle damage to effect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_BATTLE_DAMAGE_TO_EFFECT)
    effect::set_code(d, e1, EFFECT_BATTLE_DAMAGE_TO_EFFECT);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
