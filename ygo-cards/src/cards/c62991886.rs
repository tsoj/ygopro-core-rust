//! Unstable Evolution
use super::prelude::*;

pub const ID: u32 = 62991886;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk Change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(lua_fn(cond)));
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e)
fn cond(d: &mut Duel, _a: Args) -> bool {
    // return Duel.GetLP(0)~=Duel.GetLP(1)
    duel::get_lp(d, 0) != duel::get_lp(d, 1)
}
// function s.value(e,c)
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local p=e:GetHandler():GetControler()
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    // if Duel.GetLP(p)<Duel.GetLP(1-p) then
    if duel::get_lp(d, p) < duel::get_lp(d, 1 - p) {
        // return 2400
        2400
    } else if duel::get_lp(d, p) > duel::get_lp(d, 1 - p) {
        // return 1000
        1000
    } else {
        // (falls off the end: nil, which the core reads as 0)
        0
    }
}
