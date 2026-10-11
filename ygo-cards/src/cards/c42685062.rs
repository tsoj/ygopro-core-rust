//! Emissary from Pandemonium
//! (Lua: 地獄からの使い)
use super::prelude::*;

pub const ID: u32 = 42685062;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCondition(s.sumcon)
    effect::set_condition(d, e2, Some(lua_fn(sumcon)));
    // e2:SetOperation(s.sumop)
    effect::set_operation(d, e2, Some(lua_fn(sumop)));
    // e2:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumcon(e,c,minc)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc<=1 and Duel.CheckTribute(c,1)
    a.int(2) <= 1 && duel::check_tribute(d, c, 1, None, None, None, None)
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sumop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let c = a.c(8);
    // local g=Duel.SelectTribute(tp,c,1,1)
    let g = duel::select_tribute(d, tp, c, 1, 1, None, None, None, false).nil();
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, g, REASON_SUMMON | REASON_MATERIAL, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(5)
    effect::set_value(d, e1, 5);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetValue(1300)
    effect::set_value(d, e2, 1300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e3, EFFECT_SET_BASE_DEFENSE);
    // e3:SetValue(900)
    effect::set_value(d, e3, 900);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
