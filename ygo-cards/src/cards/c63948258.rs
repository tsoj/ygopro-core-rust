//! Submarine Frog
use super::prelude::*;

pub const ID: u32 = 63948258;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
