//! Constellar Omega
//! セイクリッド・オメガ
use super::prelude::*;

pub const ID: u32 = 26329679;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CONSTELLAR as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // All your "Constellar" become unaffected by spells/traps
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetHintTiming(TIMING_STANDBY_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE | TIMING_END_PHASE, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellTrapEffect()
    effect::is_spell_trap_effect(d, a.e(1))
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_CONSTELLAR),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CONSTELLAR as u16])), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3104)
        effect::set_description(d, e1, 3104);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(efilter));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
