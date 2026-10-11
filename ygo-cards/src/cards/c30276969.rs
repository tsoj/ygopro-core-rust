//! Reese the Ice Mistress
use super::prelude::*;

pub const ID: u32 = 30276969;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(s.indes)
    effect::set_value_fn(d, e1, lua_fn(indes));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.indes(e,c)`
fn indes(d: &mut Duel, a: Args) -> bool {
    // return c:IsLevelAbove(4)
    let c = a.c(1);
    card::is_level_above(d, c, 4)
}
