//! E・HERO ネクロダークマン (Elemental HERO Necroshade)
use super::prelude::*;

pub const ID: u32 = 89252153;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with no tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetTarget(aux.FieldSummonProcTg(s.nttg))
    effect::set_target(d, e1, Some(aux::field_summon_proc_tg(Some(lua_fn(nttg)), None)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    a.int(2) == 0 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.nttg(e,c)`
fn nttg(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_ELEMENTAL_HERO)
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
}
