//! Mono Synchron
use super::prelude::*;

pub const ID: u32 = 56897896;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro custom
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e1:SetOperation(s.synop)
    effect::set_operation(d, e1, Some(lua_fn(synop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SYNCHRO_MAT_RESTRICTION)
    effect::set_code(d, e4, EFFECT_SYNCHRO_MAT_RESTRICTION);
    // e4:SetValue(s.synfilter)
    effect::set_value_fn(d, e4, lua_fn(synfilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.synfilter(e,c)`
fn synfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_WARRIOR|RACE_MACHINE)
    let c = a.c(1);
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_WARRIOR | RACE_MACHINE)
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    // return #sg-1+e:GetHandler():GetLevel()==lv,true
    let sg = a.g(3);
    let lv = a.int(4);
    let h = effect::get_handler(d, a.e(0)).nil();
    let n = sg.len() as i64;
    let level = card::get_level(d, h) as i64;
    Value::Multi(vec![Value::Bool(n - 1 + level == lv), Value::Bool(true)])
}
