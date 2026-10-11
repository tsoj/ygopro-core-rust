//! Victory Dragon
//! ヴィクトリー・ドラゴン
use super::prelude::*;

pub const ID: u32 = 44910027;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --cannot special summon
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
    // --summon with 3 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,false,3,3)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // local e2=aux.AddNormalSetProcedure(c)
    let _e2 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // --tribute limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_TRIBUTE_LIMIT)
    effect::set_code(d, e3, EFFECT_TRIBUTE_LIMIT);
    // e3:SetValue(s.tlimit)
    effect::set_value_fn(d, e3, effect_card_fn(tlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --match kill
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EFFECT_MATCH_KILL)
    effect::set_code(d, e4, EFFECT_MATCH_KILL);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tlimit(e,c)`
fn tlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_DRAGON)
    !card::is_race(d, c, RACE_DRAGON as u64)
}
