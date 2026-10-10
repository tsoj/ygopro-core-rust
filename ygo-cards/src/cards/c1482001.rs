//! Clara & Rushka, the Ventriloduo
use super::prelude::*;

pub const ID: u32 = 1482001;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Normal Summoned/Set monster
    // Link.AddProcedure(c,aux.FilterBoolFunction(Card.IsSummonType,SUMMON_TYPE_NORMAL),1,1,nil,nil,s.splimit)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL as u32]))),
        1,
        Some(1),
        None,
        None,
        Some(lua_fn(splimit)),
    );
    // Cannot be Link Summoned except during Main Phase 2
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_LINK)~=SUMMON_TYPE_LINK or Duel.IsPhase(PHASE_MAIN2)
    let st = a.int(3) as u32;
    (st & SUMMON_TYPE_LINK as u32) != SUMMON_TYPE_LINK as u32 || duel::is_phase(d, PHASE_MAIN2)
}
