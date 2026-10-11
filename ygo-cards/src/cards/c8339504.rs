//! Xyz Shift
use super::prelude::*;

pub const ID: u32 = 8339504;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local rk=c:GetRank()
    // return rk>0 and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_EXTRA,0,1,nil,c,e,tp)
    let rk = card::get_rank(d, c);
    rk > 0 && duel::is_existing_matching_card(d, Some(&|d, x| spfilter1(d, x, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter1(c,tc,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, tc: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRank(tc:GetRank()) and c:IsRace(tc:GetRace()) and c:IsAttribute(tc:GetAttribute())
    //     and not c:IsCode(tc:GetCode()) and Duel.GetLocationCountFromEx(tp,tp,tc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let rank = card::get_rank(d, tc);
    if !card::is_rank(d, c, &[rank]) {
        return false;
    }
    let race = card::get_race(d, tc);
    if !card::is_race(d, c, race) {
        return false;
    }
    let attr = card::get_attribute(d, tc);
    if !card::is_attribute(d, c, attr) {
        return false;
    }
    let codes = card::get_code(d, tc);
    if card::is_code(d, c, &codes) {
        return false;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), tc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c,tc,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, tc: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRank(tc:GetPreviousRankOnField()) and c:IsRace(tc:GetPreviousRaceOnField()) and c:IsAttribute(tc:GetPreviousAttributeOnField())
    //     and not c:IsCode(tc:GetPreviousCodeOnField()) and Duel.GetLocationCountFromEx(tp,tp,tc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let rank = card::get_previous_rank_on_field(d, tc);
    if !card::is_rank(d, c, &[rank]) {
        return false;
    }
    let race = card::get_previous_race_on_field(d, tc);
    if !card::is_race(d, c, race) {
        return false;
    }
    let attr = card::get_previous_attribute_on_field(d, tc);
    if !card::is_attribute(d, c, attr) {
        return false;
    }
    let codes = card::get_previous_code_on_field(d, tc);
    if card::is_code(d, c, &codes) {
        return false;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), tc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.CheckReleaseGroup(tp,s.cfilter,1,nil,e,tp)
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::check_release_group(d, tp, Some(&|d, x| cfilter(d, x, a.e, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local g=Duel.SelectReleaseGroup(tp,s.cfilter,1,1,nil,e,tp)
    let g = duel::select_release_group(d, tp, Some(&|d, x| cfilter(d, x, a.e, tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(g,REASON_COST)
    duel::release(d, &g, REASON_COST, None);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,tc,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter2(d, x, tc, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
            // if c:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, c, a.e) {
                // c:CancelToGrave()
                card::cancel_to_grave(d, c, true);
                // Duel.Overlay(sc,c)
                duel::overlay(d, sc, CardSet::from_card(c), false);
            }
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetLabel(fid)
            effect::set_label(d, e1, &[fid as i64]);
            // e1:SetLabelObject(sc)
            effect::set_label_object(d, e1, Value::Card(sc));
            // e1:SetCondition(s.tgcon)
            effect::set_condition(d, e1, Some(condition(tgcon)));
            // e1:SetOperation(s.tgop)
            effect::set_operation(d, e1, Some(operation(tgop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then
    let flag_label = card::get_flag_effect_label(d, tc, ID).first().copied();
    if flag_label != Some(effect::get_label(d, a.e)) {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoGrave(e:GetLabelObject(),REASON_EFFECT)
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
}
