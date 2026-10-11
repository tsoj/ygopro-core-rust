//! Urgent Tuning
use super::prelude::*;

pub const ID: u32 = 94634433;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e1:SetCondition(s.sccon)
    effect::set_condition(d, e1, Some(condition(sccon)));
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.sccon(e,tp,eg,ep,ev,re,r,rp)`
fn sccon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
/// `Card.IsSynchroSummonable` with the extra arguments `(nil, nil)`
fn scfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_synchro_summonable(d, c, None, None, 0, 0)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&scfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil,nil)
    let g = duel::get_matching_group(d, Some(&scfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),nil)
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    }
}
