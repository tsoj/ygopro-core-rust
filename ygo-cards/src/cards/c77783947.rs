//! Yang Zing Unleashed
//! 竜星の極み
use super::prelude::*;

pub const ID: u32 = 77783947;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YANG_ZING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // must attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // synchro effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e4, 0, Some((TIMING_BATTLE_START | TIMING_BATTLE_END) as u32));
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.sccon)
    effect::set_condition(d, e4, Some(condition(sccon)));
    // e4:SetCost(s.sccost)
    effect::set_cost(d, e4, Some(cost_target(sccost)));
    // e4:SetTarget(s.sctg)
    effect::set_target(d, e4, Some(cost_target(sctg)));
    // e4:SetOperation(s.scop)
    effect::set_operation(d, e4, Some(operation(scop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sccon(e,tp,eg,ep,ev,re,r,rp)`
fn sccon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() or Duel.IsBattlePhase()
    duel::is_main_phase(d, None) || duel::is_battle_phase(d, None)
}
/// `s.sccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sccost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YANG_ZING) and c:IsMonster()
    card::is_set_card(d, c, &[SET_YANG_ZING as u16]) && card::is_monster(d, c)
}
/// `s.checkaddition(tp,sg,sc)`
fn checkaddition(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(s.mfilter,1,nil)
    let sg = a.g(1);
    group::is_exists(d, &sg, &mfilter, 1, NO_EX)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // Synchro.CheckAdditional=s.checkaddition
        synchro::set_check_additional(d, Some(lua_fn(checkaddition)));
        // local res=Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Synchro.CheckAdditional=s.checkaddition
    synchro::set_check_additional(d, Some(lua_fn(checkaddition)));
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst())
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    } else {
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
    }
}
