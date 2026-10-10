//! Agave Dragon
use super::prelude::*;

pub const ID: u32 = 2411269;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.NOT(aux.FilterBoolFunctionEx(Card.IsType,TYPE_TOKEN)),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| !card::is_type_ctx(d, c, TYPE_TOKEN, sc, st, Some(tp)))), 2, None, None, None, None);
    // apply effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_RECOVER+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_RECOVER + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return #g>0 and (g:FilterCount(Card.IsRace,nil,RACE_SEASERPENT)<#g or Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)) end
    if a.chk == Some(0) {
        let seaserpent = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_SEASERPENT), NO_EX);
        return !g.is_empty()
            && (seaserpent < g.len() || duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,g:FilterCount(Card.IsRace,nil,RACE_DRAGON)*100)
    let dragon = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_DRAGON), NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (dragon * 100) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,g:FilterCount(Card.IsRace,nil,RACE_WYRM)*400)
    let wyrm = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_WYRM), NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, (wyrm * 400) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct1=g:FilterCount(Card.IsRace,nil,RACE_DRAGON)
    let ct1 = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_DRAGON), NO_EX);
    // local ct2=g:FilterCount(Card.IsRace,nil,RACE_DINOSAUR)
    let ct2 = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_DINOSAUR), NO_EX);
    // local ct3=g:FilterCount(Card.IsRace,nil,RACE_SEASERPENT)
    let ct3 = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_SEASERPENT), NO_EX);
    // local ct4=g:FilterCount(Card.IsRace,nil,RACE_WYRM)
    let ct4 = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_WYRM), NO_EX);
    // if ct1>0 then
    if ct1 > 0 {
        // Duel.Damage(1-tp,ct1*100,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, (ct1 * 100) as i64, REASON_EFFECT, false, None);
    }
    // if ct2>0 and c:IsFaceup() and c:IsRelateToEffect(e) then
    if ct2 > 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct2*200)
        effect::set_value(d, e1, (ct2 * 200) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // local og=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let og = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if ct3>0 and #og>0 then
    if ct3 > 0 && !og.is_empty() {
        // for tc in aux.Next(og) do
        for tc in group::iter(d, &og) {
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            // e2:SetValue(ct3*-300)
            effect::set_value(d, e2, ct3 as i64 * -300);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // if ct4>0 then
    if ct4 > 0 {
        // Duel.Recover(tp,ct4*400,REASON_EFFECT)
        duel::recover(d, a.tp, (ct4 * 400) as i64, REASON_EFFECT, false, None);
    }
}
