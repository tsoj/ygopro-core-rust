//! High Tide on Fire Island
use super::prelude::*;

pub const ID: u32 = 32542011;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(7) and c:IsAttribute(ATTRIBUTE_WATER|ATTRIBUTE_FIRE)
    card::is_faceup(d, c) && card::is_level_above(d, c, 7) && card::is_attribute(d, c, ATTRIBUTE_WATER | ATTRIBUTE_FIRE)
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// s.cfilter2(c,att)
fn cfilter2(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
// s.spfilter(c,tid,e,tp)
fn spfilter(d: &mut Duel, c: CardId, tid: i32, e: EffectId, tp: u8) -> bool {
    // local re=c:GetReasonEffect()
    let re = card::get_reason_effect(d, c);
    // return c:GetTurnID()==tid and c:IsReason(REASON_COST) and re and re:IsActivated() and re:IsMonsterEffect()
    //   and c:IsAttribute(ATTRIBUTE_WATER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    if card::get_turn_id(d, c) as i32 != tid || !card::is_reason(d, c, REASON_COST) {
        return false;
    }
    let Some(re) = re else { return false };
    effect::is_activated(d, re)
        && effect::is_monster_effect(d, re)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local b1=Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil,ATTRIBUTE_WATER)
    //   and ft>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,Duel.GetTurnCount(),e,tp)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| cfilter2(d, c, ATTRIBUTE_WATER)), tp, LOCATION_MZONE, 0, 1, NO_EX) && ft > 0 && {
        let tid = duel::get_turn_count(d, None);
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, tid, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    };
    // local b2=Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil,ATTRIBUTE_FIRE)
    //   and Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| cfilter2(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local loc=b2 and LOCATION_MZONE or 0
    let loc = if b2 { LOCATION_MZONE } else { 0 };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,loc,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, loc, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local break_chk=false
    let mut break_chk = false;
    // WATER: Special Summon from the GY
    // if Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil,ATTRIBUTE_WATER) then
    if duel::is_existing_matching_card(d, Some(&|d, c| cfilter2(d, c, ATTRIBUTE_WATER)), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // local ct=Duel.GetMatchingGroupCount(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,Duel.GetTurnCount(),e,tp)
        let tid = duel::get_turn_count(d, None);
        let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, tid, a.e, tp));
        let ct = duel::get_matching_group_count(d, Some(&nv), tp, LOCATION_GRAVE, 0, NO_EX) as i32;
        // ft=math.min(ft,ct)
        ft = ft.min(ct);
        // if ft>0 then
        if ft > 0 {
            // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
            if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
                ft = 1;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,ft,ft,nil,Duel.GetTurnCount(),e,tp)
            let tid = duel::get_turn_count(d, None);
            let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, tid, a.e, tp));
            let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE, 0, ft as u16, ft as u16, false, NO_EX).nil();
            // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
            if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // break_chk=true
                break_chk = true;
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // local dg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
                let dg = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
                // if #dg>0 then
                if !dg.is_empty() {
                    // Duel.HintSelection(dg,true)
                    duel::hint_selection(d, dg.clone(), true);
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Destroy(dg,REASON_EFFECT)
                    duel::destroy(d, dg, REASON_EFFECT, None, None);
                }
            }
        }
    }
    // FIRE: Destroy 1 monster on the field
    // if Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil,ATTRIBUTE_FIRE) then
    if duel::is_existing_matching_card(d, Some(&|d, c| cfilter2(d, c, ATTRIBUTE_FIRE)), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // if Duel.Destroy(g,REASON_EFFECT)>0 and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0 then
            if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && duel::get_field_group_count(d, tp, LOCATION_HAND, 0) > 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.DiscardHand(tp,nil,1,1,REASON_DISCARD|REASON_EFFECT)
                duel::discard_hand(d, tp, None, 1, 1, REASON_DISCARD | REASON_EFFECT, NO_EX);
            }
        }
    }
}
