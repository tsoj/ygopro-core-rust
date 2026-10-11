//! Dinomist Rush
use super::prelude::*;

pub const ID: u32 = 41554273;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Dinomist" monster from your Deck
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DINOMIST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // --Destroy it during the End Phase
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE | PHASE_END);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCondition(s.descon)
            effect::set_condition(d, e1, Some(condition(descon)));
            // e1:SetOperation(s.desop)
            effect::set_operation(d, e1, Some(operation(desop)));
            // e1:SetLabelObject(sc)
            effect::set_label_object(d, e1, Value::Card(sc));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
            // --Unaffected by other cards' effects
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(3100)
            effect::set_description(d, e2, 3100);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE | EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
            // e2:SetValue(s.efilter)
            effect::set_value_fn(d, e2, lua_fn(efilter));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)>0 then
    if card::get_flag_effect(d, tc, ID) > 0 {
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetHandler()
    effect::get_owner(d, a.e(1)) != effect::get_handler(d, a.e(0))
}
