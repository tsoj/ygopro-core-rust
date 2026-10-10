//! Assault Mode Activate
use super::prelude::*;

pub const ID: u32 = 80280737;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSynchroMonster() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    if !card::is_synchro_monster(d, c) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_ASSAULT_MODE) and c.assault_mode and c.assault_mode==code and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16])
        && crate::card_info(d, c).assault_mode == Some(code)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local sc=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let sc = group::get_first(d, &g).nil();
    // Duel.SetTargetParam(sc:GetCode())
    let code = card::get_code(d, sc)[0];
    duel::set_target_param(d, code);
    // Duel.Release(sc,REASON_COST)
    duel::release(d, CardSet::from_card(sc), REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP_ATTACK)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, true, false, POS_FACEUP_ATTACK as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
