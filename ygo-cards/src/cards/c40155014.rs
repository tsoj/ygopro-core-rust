//! Centur-Ion Phalanx
//! 騎士魔防陣
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 40155014;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CENTUR_ION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 Monster Card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Centurion" Synchro Monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmvfilter(c)
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and s.rmvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && rmvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmvfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmvfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmvfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmvfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, LOCATION_REMOVED as i32);
    true
}
// function s.rmvop(e,tp,eg,ep,ev,re,r,rp)
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_REMOVED) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
    {
        // local turn_ct=Duel.GetTurnCount()
        let turn_ct = duel::get_turn_count(d, None);
        // local reset_ct=Duel.GetCurrentPhase()<=PHASE_STANDBY and 2 or 1
        let reset_ct: u16 = if duel::get_current_phase(d) <= PHASE_STANDBY { 2 } else { 1 };
        // Special Summon it to its owner's field during the Standby Phase of the next turn
        // aux.DelayedOperation(tc,PHASE_STANDBY,id,e,tp,
        //     function(ag)
        //         Duel.SpecialSummon(ag,0,tp,ag:GetFirst():GetOwner(),false,false,POS_FACEUP)
        //     end,
        //     function()
        //         return Duel.GetTurnCount()==turn_ct+1
        //     end,
        //     nil,reset_ct
        // )
        aux::delayed_operation(
            d,
            tc,
            PHASE_STANDBY,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(move |d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                let first = group::get_first(d, ag).nil();
                let owner = card::get_owner(d, first);
                duel::special_summon(d, ag.clone(), 0, tp, owner, false, false, POS_FACEUP as u8, None);
                true
            })),
            Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_ct + 1)),
            None,
            Some(reset_ct),
            None,
            None,
        );
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsType(TYPE_SYNCHRO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // It loses 1500 ATK
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-1500)
        effect::set_value(d, e1, -1500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
