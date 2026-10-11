//! Cybernetic Revolution
use super::prelude::*;

pub const ID: u32 = 18597560;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 fusion monster, that lists "Cyber Dragon" as material, from extra deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(TIMING_BATTLE_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE | TIMING_END_PHASE, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_CYBER_DRAGON) and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_code(d, c, &[CARD_CYBER_DRAGON])
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.cfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local g=Duel.SelectReleaseGroup(tp,s.cfilter,1,1,nil,e,tp)
    let g = duel::select_release_group(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.filter(c,e,tp,mc)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsType(TYPE_FUSION) and c:ListsArchetypeAsMaterial(SET_CYBER_DRAGON) and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_FUSION)
        && card::lists_archetype_as_material(d, c, &[SET_CYBER_DRAGON as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, |m| m.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
            // Cannot attack directly
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
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // Destroy it during next end phase
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e2:SetCondition(s.descon)
            effect::set_condition(d, e2, Some(condition(descon)));
            // e2:SetOperation(s.desop)
            effect::set_operation(d, e2, Some(operation(desop)));
            // e2:SetReset(RESET_PHASE|PHASE_END,2)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetLabel(Duel.GetTurnCount())
            effect::set_label(d, e2, &[duel::get_turn_count(d, None) as i64]);
            // e2:SetLabelObject(tc)
            effect::set_label_object(d, e2, Value::Card(tc));
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return Duel.GetTurnCount()~=e:GetLabel() and tc:GetFlagEffect(id)~=0
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e) && card::get_flag_effect(d, tc, ID) != 0
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
