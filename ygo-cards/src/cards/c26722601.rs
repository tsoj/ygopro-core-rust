//! Steelswarm Gatekeeper
//! インヴェルズの門番
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STEELSWARM], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 26722601;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // extra summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &a.eg.clone().nil());
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if ec~=e:GetHandler() and ec:IsSetCard(SET_STEELSWARM) and ec:IsTributeSummoned() then
    if ec != Some(c) && card::is_set_card(d, ec.nil(), &[SET_STEELSWARM as u16]) && card::is_tribute_summoned(d, ec.nil()) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
        // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
        effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_EXTRA_SET_COUNT)
        effect::set_code(d, e2, EFFECT_EXTRA_SET_COUNT);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
