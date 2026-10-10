//! Yoko, the Graceful Mayakashi
use super::prelude::*;

pub const ID: u32 = 3486020;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy 1 monster if Special Summoned from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetPreviousLocation()==LOCATION_GRAVE
    let c = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, c) == LOCATION_GRAVE
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsDestructable,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_destructable(d, c, None)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsDestructable,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_destructable(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
// function s.spfilter(c,tp)
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and (c:GetPreviousTypeOnField()&TYPE_SYNCHRO)~=0
    //     and c:GetOriginalLevel()==11 and (c:IsReason(REASON_BATTLE) or c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp)
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_controler(d, c, tp)
        && (card::get_previous_type_on_field(d, c) & TYPE_SYNCHRO) != 0
        && card::get_original_level(d, c) == 11
        && (card::is_reason(d, c, REASON_BATTLE)
            || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.spfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| spfilter(d, c, tp), 1, NO_EX)
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and c:IsRace(RACE_ZOMBIE)
    card::is_able_to_remove(d, c, None, None, None) && card::is_race(d, c, RACE_ZOMBIE)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.rmfilter),tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(rmfilter)),
        tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    // if #g>0 and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if !g.is_empty()
        && duel::remove(d, &g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
