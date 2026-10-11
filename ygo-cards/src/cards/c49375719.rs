//! Cyber Tutu
use super::prelude::*;

pub const ID: u32 = 49375719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.dacon)
    effect::set_condition(d, e1, Some(lua_fn(dacon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,atk)
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFacedown() or c:GetAttack()<=atk
    card::is_facedown(d, c) || card::get_attack(d, c) <= atk
}
// function s.dacon(e)
fn dacon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return not Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil,e:GetHandler():GetAttack())
    let p = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    let atk = card::get_attack(d, h);
    !duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, atk)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
