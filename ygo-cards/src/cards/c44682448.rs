//! Garbage Lord
use super::prelude::*;

pub const ID: u32 = 44682448;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e2:SetValue(s.xyzlimit)
    effect::set_value_fn(d, e2, lua_fn(xyzlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and Duel.CheckLPCost(c:GetControler(),2000)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::check_lp_cost(d, tp, 2000)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // Duel.PayLPCost(tp,2000)
    duel::pay_lp_cost(d, a.p(1), 2000);
}
/// `s.xyzlimit(e,c)`
fn xyzlimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else { return false };
    // return not c:IsAttribute(ATTRIBUTE_DARK)
    !card::is_attribute(d, c, ATTRIBUTE_DARK)
}
