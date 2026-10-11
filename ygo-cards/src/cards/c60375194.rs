//! Vendread Daybreak
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 60375194;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Choose 1 Ritual Summoned "Vendread" monster you control, and destroy all cards on the field, except the chosen monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0))),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.vendreadfilter(c)
fn vendreadfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSummoned() and c:IsSetCard(SET_VENDREAD) and c:IsFaceup()
    card::is_ritual_summoned(d, c) && card::is_set_card(d, c, &[SET_VENDREAD as u16]) && card::is_faceup(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.vendreadfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&vendreadfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g-2,tp,0)
    let count = (g.len() as i64 - 2) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), count, tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local sc=Duel.SelectMatchingCard(tp,s.vendreadfilter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&vendreadfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and Group.FromCards(sc,c) or sc
    let mut exc = CardSet::from_card(sc);
    if card::is_relate_to_effect(d, c, e) {
        group::add_card(&mut exc, &CardSet::from_card(c));
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, (&exc).into());
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    // It cannot attack directly while it remains face-up on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3207)
    effect::set_description(d, e1, 3207);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
}
