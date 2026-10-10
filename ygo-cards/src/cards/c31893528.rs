//! Spirit Message "I"
use super::prelude::*;

pub const ID: u32 = 31893528;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e1, EFFECT_CANNOT_SSET);
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
/// `s.spcost(e,c,tp,sumtype)`
fn spcost(_d: &mut Duel, a: Args) -> bool {
    // return sumtype==SUMMON_TYPE_SPECIAL+181
    a.int(3) == (SUMMON_TYPE_SPECIAL + 181) as i64
}
