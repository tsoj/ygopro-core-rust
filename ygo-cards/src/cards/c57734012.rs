//! Rank-Up-Magic - The Seventh One
use super::prelude::*;

pub const ID: u32 = 57734012;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_NUMBER_C], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.activate)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.GetFlagEffect(tp,id)==0 and Duel.IsPhase(PHASE_DRAW) and c:IsReason(REASON_RULE)
    duel::get_flag_effect(d, a.tp, ID) == 0 && duel::is_phase(d, PHASE_DRAW) && card::is_reason(d, c, REASON_RULE)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PUBLIC)
        effect::set_code(d, e1, EFFECT_PUBLIC);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_MAIN1,EFFECT_FLAG_CLIENT_HINT,1,0,66)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_MAIN1, EFFECT_FLAG_CLIENT_HINT, 1, 0, 66);
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity()
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return e:GetHandler():GetFlagEffect(id)~=0
        let c = effect::get_handler(d, a.e).nil();
        return card::get_flag_effect(d, c, ID) != 0;
    }
    true
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local m=c:GetMetatable(true)
    // if not m then return false end
    // (a card whose script has no `xyz_number` fails below as well)
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)<=0 then
    //     return false
    // end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // local no=m.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and no and no>=101 and no<=107 and c:IsSetCard(SET_NUMBER)
    //     and not c:IsSetCard(SET_NUMBER_C) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (aux.CheckSummonGate(tp,2) or c:IsLocation(LOCATION_GRAVE))
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,no,pg)
    let pg_ok = pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c));
    pg_ok
        && no.is_some_and(|no| (101..=107).contains(&no))
        && card::is_set_card(d, c, &[SET_NUMBER as u16])
        && !card::is_set_card(d, c, &[SET_NUMBER_C as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (aux::check_summon_gate(d, tp, 2) || card::is_location(d, c, LOCATION_GRAVE))
        && duel::is_existing_matching_card(d, Some(&|d, mc| filter2(d, mc, e, tp, c, no)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.filter2(c,e,tp,mc,no,pg)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, no: Option<u32>) -> bool {
    // if c.rum_limit then return false end
    if crate::card_info(d, c).rum_limit.is_some() {
        return false;
    }
    // return c.xyz_number==no and c:IsSetCard(SET_NUMBER_C) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    crate::card_info(d, c).xyz_number == no
        && card::is_set_card(d, c, &[SET_NUMBER_C as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ as u32, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_GRAVE end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_GRAVE;
    }
    if a.chk == Some(0) {
        // return Duel.IsPlayerCanSpecialSummonCount(tp,2) and Duel.GetFlagEffect(tp,id)==0
        //     and Duel.IsExistingMatchingCard(s.filter1,tp,loc,0,1,nil,e,tp)
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_flag_effect(d, tp, ID) == 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, a.e, tp)), tp, loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, loc as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, tp, ID) != 0 {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,0,0,0)
    duel::register_flag_effect(d, tp, ID, 0, 0, 0, 0);
    // local loc=LOCATION_EXTRA
    let mut loc = LOCATION_EXTRA;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then loc=loc|LOCATION_GRAVE end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        loc |= LOCATION_GRAVE;
    }
    // if loc==0 then return end
    if loc == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter1),tp,loc,0,1,1,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, c| filter1(d, c, e, tp));
    let g1 = duel::select_matching_card(d, tp, Some(&nv), tp, loc, 0, 1, 1, false, NO_EX).nil();
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1);
    // if tc1 and Duel.SpecialSummon(tc1,0,tp,tp,false,false,POS_FACEUP)~=0 then
    let Some(tc1) = tc1 else { return };
    if duel::special_summon(d, tc1, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(tc1),tp,nil,nil,REASON_XYZ)
        let _pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc1)), tp, None, None, REASON_XYZ);
        // local m=tc1:GetMetatable(true)
        // if not m then return end
        let no = crate::card_info(d, tc1).xyz_number;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,tc1,m.xyz_number,pg)
        let g2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp, tc1, no)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // local tc2=g2:GetFirst()
        // if tc2 then
        if let Some(tc2) = group::get_first(d, &g2) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // tc2:SetMaterial(g1)
            card::set_material(d, tc2, g1.clone());
            // Duel.Overlay(tc2,g1)
            duel::overlay(d, tc2, g1.clone(), false);
            // Duel.SpecialSummon(tc2,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc2, SUMMON_TYPE_XYZ as u32, tp, tp, false, false, POS_FACEUP as u8, None);
            // tc2:CompleteProcedure()
            card::complete_procedure(d, tc2);
        }
    }
}
