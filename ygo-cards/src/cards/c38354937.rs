//! Locomotion R-Genex
use super::prelude::*;

pub const ID: u32 = 38354937;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GENEX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GENEX),1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_DARK),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GENEX as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.ctcon)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasLevel()
    card::is_faceup(d, c) && card::has_level(d, c)
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return;
    }
    // local sg=g:GetMaxGroup(Card.GetLevel)
    let mut sg = group::get_max_group(d, &g, &|d, c| card::get_level(d, c) as i64).map_or_else(CardSet::new, |x| x.0);
    // if #sg>1 then
    if group::get_count(&sg) > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        // sg=sg:Select(tp,1,1,nil)
        sg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    }
    // local tc=sg:GetFirst()
    let tc = group::get_first(d, &sg).nil();
    // Duel.GetControl(tc,tp)
    duel::get_control(d, CardSet::from_card(tc), tp, 0, 0, None, None);
}
