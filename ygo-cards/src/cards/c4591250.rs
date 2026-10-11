//! Amazoness Empress
use super::prelude::*;

pub const ID: u32 = 4591250;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], listed_names: &[15951532], material_setcode: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,15951532,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_AMAZONESS))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(15951532),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_AMAZONESS as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, 0);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pierce
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_AMAZONESS))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_AMAZONESS as u16]))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.indtg(e,c)
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMAZONESS) and c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && c != h
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned()
    //     and (c:IsReason(REASON_BATTLE) or (c:GetReasonPlayer()~=tp and c:IsReason(REASON_EFFECT)))
    //     and c:IsPreviousPosition(POS_FACEUP)
    card::is_fusion_summoned(d, c)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::get_reason_player(d, c) != a.tp && card::is_reason(d, c, REASON_EFFECT)))
        && card::is_previous_position(d, c, POS_FACEUP)
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(15951532) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[15951532]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| filter(d, c, e, tp))), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
