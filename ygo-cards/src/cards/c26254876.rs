//! Gemini Lancer
//! デュアル・ランサー
use super::prelude::*;

pub const ID: u32 = 26254876;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Inflict piercing damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
