//! Sangen Kaiho
use super::prelude::*;

pub const ID: u32 = 25388971;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TENPAI_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // End the Main Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.phcon)
    effect::set_condition(d, e1, Some(condition(phcon)));
    // e1:SetOperation(s.phop)
    effect::set_operation(d, e1, Some(operation(phop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon any number of "Tenpai Dragon" monsters from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function() return Duel.GetFlagEffect(0,id)+Duel.GetFlagEffect(1,id)>=3 end)
    effect::set_condition(d, e2, Some(condition(flag_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Count attacks declared this turn
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(function(_,_,_,ep) Duel.RegisterFlagEffect(ep,id,RESET_PHASE|PHASE_END,0,1) end)
        effect::set_operation(d, ge1, Some(operation(ge1_op)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `function(_,_,_,ep) Duel.RegisterFlagEffect(ep,id,RESET_PHASE|PHASE_END,0,1) end`
fn ge1_op(d: &mut Duel, a: &EffArgs) {
    duel::register_flag_effect(d, a.ep, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `function() return Duel.GetFlagEffect(0,id)+Duel.GetFlagEffect(1,id)>=3 end`
fn flag_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_flag_effect(d, 0, ID) + duel::get_flag_effect(d, 1, ID) >= 3
}
/// `s.phconfilter(c)`
fn phconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_DRAGON)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.phcon(e,tp,eg,ep,ev,re,r,rp)`
fn phcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not Duel.IsMainPhase() then return false end
    if !duel::is_main_phase(d, None) {
        return false;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // return #g>0 and g:FilterCount(s.phconfilter,nil)==#g and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>#g
    !g.is_empty()
        && group::filter_count(d, &g, &phconfilter, NO_EX) == g.len()
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) as usize > g.len()
}
/// `s.phop(e,tp,eg,ep,ev,re,r,rp)`
fn phop(d: &mut Duel, a: &EffArgs) {
    // Duel.SkipPhase(1-tp,Duel.GetCurrentPhase(),RESET_PHASE|PHASE_END,1)
    let phase = duel::get_current_phase(d);
    duel::skip_phase(d, 1 - a.tp, phase, RESET_PHASE | PHASE_END, 1, 0);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TENPAI_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TENPAI_DRAGON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // local ct=math.min(#g,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let ct = (g.len() as i32).min(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0);
    // if ct<=0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if ct <= 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    let mut ct = ct;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,1,ct,nil)
    let sg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
