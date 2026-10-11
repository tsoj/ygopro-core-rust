//! Lane Restriction
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 47914440;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can Special Summon 1 of your banished monsters to your rightmost Main Monster Zone
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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent would Normal or Special Summon a monster(s) to their Main Monster Zone, they must use the leftmost unused zone(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e2, EFFECT_FORCE_MZONE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(s.forcemzoneval)
    effect::set_value_fn(d, e2, lua_fn(|d, a| forcemzoneval(d, &a)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `local offset=Duel.IsDuelType(DUEL_3_COLUMNS_FIELD) and 1 or 0`
fn offset(d: &mut Duel) -> u32 {
    if duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD) { 1 } else { 0 }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spfilter(c,e,tp,rightmost_zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rightmost_zone: u32) -> bool {
    // return c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,rightmost_zone)
    card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(rightmost_zone))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rightmost_zone=1<<RIGHTMOST_SEQ
    let rightmost_zone: u32 = 1 << (4 - offset(d));
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,nil,rightmost_zone)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), None, Some(rightmost_zone)).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_REMOVED,0,nil,e,tp,rightmost_zone)
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, rightmost_zone)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP,rightmost_zone)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, Some(rightmost_zone));
        }
    }
}
/// `s.forcemzoneval(e,fp,rp,r)`
fn forcemzoneval(d: &mut Duel, a: &Args) -> i64 {
    // Needs to somehow not affect Normal Sets, no way to do that atm
    // local opp=1-e:GetHandlerPlayer()
    let opp = 1 - effect::get_handler_player(d, a.e(0));
    let (leftmost, rightmost) = (offset(d), 4 - offset(d));
    // for seq=LEFTMOST_SEQ,RIGHTMOST_SEQ do
    for seq in leftmost..=rightmost {
        // if Duel.CheckLocation(opp,LOCATION_MZONE,seq) then
        if duel::check_location(d, opp, LOCATION_MZONE, seq) {
            // return (1<<seq)|ZONES_EMZ|(ZONES_MMZ<<16)
            return ((1 << seq) | ZONES_EMZ | (ZONES_MMZ << 16)) as i64;
        }
    }
    // return ZONES_EMZ|(ZONES_MMZ<<16)
    (ZONES_EMZ | (ZONES_MMZ << 16)) as i64
}
