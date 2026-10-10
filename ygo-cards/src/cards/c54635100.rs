//! Linkerbell
use super::prelude::*;

pub const ID: u32 = 54635100;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // link summon
    // Link.AddProcedure(c,nil,2,2)
    link::add_procedure(d, c, None, 2, Some(2), None, None, None);
    // splimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e1, EFFECT_SPSUMMON_COST);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(lua_fn(spcost)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcost(e,c,tp,st)`
fn spcost(d: &mut Duel, a: Args) -> bool {
    // if (st&SUMMON_TYPE_LINK)~=SUMMON_TYPE_LINK then return true end
    let st = a.int(3) as u64;
    if (st & SUMMON_TYPE_LINK as u64) != SUMMON_TYPE_LINK as u64 {
        return true;
    }
    // return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>=3
    let tp = a.p(2);
    duel::get_field_group_count(d, tp, LOCATION_EXTRA, 0) - duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA) >= 3
}
