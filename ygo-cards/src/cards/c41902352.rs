//! Blackwing - Kochi the Daybreak
use super::prelude::*;

pub const ID: u32 = 41902352;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // e1:SetCondition(s.synlimit)
    effect::set_condition(d, e1, Some(condition(synlimit)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e2, EFFECT_SPSUMMON_COST);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(lua_fn(spcost)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.synlimit(e)`
fn synlimit(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSpecialSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_special_summoned(d, h)
}
/// `s.spcost(e,c,tp,sumtype)`
fn spcost(_d: &mut Duel, a: Args) -> bool {
    // return sumtype~=SUMMON_TYPE_SPECIAL+182
    a.int(3) != (SUMMON_TYPE_SPECIAL + 182) as i64
}
