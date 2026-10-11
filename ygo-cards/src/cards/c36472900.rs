//! Sonic Chick
use super::prelude::*;

pub const ID: u32 = 36472900;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
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
/// `s.indes(e,c)`: `return c:IsAttackAbove(1900)`
fn indes(d: &mut Duel, a: Args) -> bool {
    card::is_attack_above(d, a.c(1), 1900)
}
