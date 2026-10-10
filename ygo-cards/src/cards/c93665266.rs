//! Crystron Quan
use super::prelude::*;

pub const ID: u32 = 93665266;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon a monster from the hand and Synchro Summon a Machine monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
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
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_CHAINING) and Duel.IsTurnPlayer(1-tp)
    //     and (Duel.IsMainPhase() or Duel.IsBattlePhase())
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_CHAINING) && duel::is_turn_player(d, 1 - a.tp) && (duel::is_main_phase(d, None) || duel::is_battle_phase(d, None))
}
/// `s.scfilter1(c,e,tp,mc)`
fn scfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // local mg=Group.FromCards(c,mc)
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(mc));
    // return not c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.syncfilter,tp,LOCATION_EXTRA,0,1,nil,mg)
    !card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| syncfilter(d, cc, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.syncfilter(c,mg)`
fn syncfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsSynchroSummonable(nil,mg)
    card::is_race(d, c, RACE_MACHINE) && card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.scfilter1,tp,LOCATION_HAND,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_special_summon_count(d, a.tp, 2)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| scfilter1(d, c, a.e, a.tp, h)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.scfilter1,tp,LOCATION_HAND,0,1,1,nil,e,tp,c)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| scfilter1(d, cc, a.e, a.tp, c)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if not tc or not Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then return end
    let Some(tc) = tc else {
        return;
    };
    if !duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local mg=Group.FromCards(c,tc)
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(tc));
    // local g=Duel.GetMatchingGroup(s.syncfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let g = duel::get_matching_group(d, Some(&|d, cc| syncfilter(d, cc, &mg)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),nil,mg)
        let first = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, a.tp, first, None, Some(mg), 0, 0);
    }
}
