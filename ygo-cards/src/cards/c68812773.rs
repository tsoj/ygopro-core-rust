//! Lillybot
use super::prelude::*;

pub const ID: u32 = 68812773;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY, SET_PHOTON], listed_names: &[71071546], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Orbital 7" from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP)
    effect::set_code(d, e2, EVENT_FLIP);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon "Photon" and/or "Galaxy" monsters from your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg2)
    effect::set_target(d, e3, Some(cost_target(sptg2)));
    // e3:SetOperation(s.spop2)
    effect::set_operation(d, e3, Some(operation(spop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(71071546) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    card::is_code(d, c, &[71071546])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // if Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)~=0 and tc:IsFacedown() then
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None) != 0 && card::is_facedown(d, tc) {
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - a.tp, tc);
        }
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_GALAXY,SET_PHOTON}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GALAXY as u16, SET_PHOTON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.cfilter(c,ft,tp)
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE)
    //     and (ft>0 or (c:IsControler(tp) and c:GetSequence()<5)) and (c:IsControler(tp) or c:IsFaceup())
    card::is_race(d, c, RACE_MACHINE)
        && (ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,ft,tp) end
    if a.chk == Some(0) {
        return ft > -1 && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let mut ct = duel::get_matching_group_count(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX) as i64;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,ct,false,nil,nil,ft,tp)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, ct as usize, false, None, NO_EX);
    // ct=Duel.Release(rg,REASON_COST)
    ct = duel::release(d, rg, REASON_COST, None) as i64;
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    true
}
// function s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, 0, 0);
    true
}
// function s.spop2(e,tp,eg,ep,ev,re,r,rp)
fn spop2(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local ct=e:GetLabel()
    let mut ct = effect::get_label(d, a.e);
    // if ft<ct then ct=ft end
    if (ft as i64) < ct {
        ct = ft as i64;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local dg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,ct,ct,nil,e,tp)
    let dg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.SpecialSummon(dg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &dg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
