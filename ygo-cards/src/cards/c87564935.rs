//! Metallizing Parasite - Soltite
use super::prelude::*;

pub const ID: u32 = 87564935;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,nil,true)
    aux::add_union_procedure(d, c, None, true, None);
    // Equip effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e1, Some(aux::is_union_state()));
    // e1:SetValue(s.efilter1)
    effect::set_value_fn(d, e1, lua_fn(efilter1));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetCondition(aux.IsUnionState)
    effect::set_condition(d, e2, Some(aux::is_union_state()));
    // e2:SetValue(s.efilter2)
    effect::set_value_fn(d, e2, lua_fn(efilter2));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.efilter1(e,re,rp)`
fn efilter1(d: &mut Duel, a: Args) -> bool {
    // return rp==1-e:GetHandlerPlayer() and re:IsMonsterEffect()
    let e = a.e(0);
    let re = a.v(1).as_effect().nil();
    let rp = a.p(2);
    let hp = effect::get_handler_player(d, e);
    rp as i64 == 1 - hp as i64 && effect::is_monster_effect(d, re)
}
/// `s.efilter2(e,te)`
fn efilter2(d: &mut Duel, a: Args) -> bool {
    // return te:GetHandlerPlayer()~=e:GetHandlerPlayer() and te:IsMonsterEffect()
    let e = a.e(0);
    let te = a.v(1).as_effect().nil();
    let tp_te = effect::get_handler_player(d, te);
    let hp = effect::get_handler_player(d, e);
    tp_te != hp && effect::is_monster_effect(d, te)
}
