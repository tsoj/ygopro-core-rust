//! Worldsea Dragon Zealantis
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 45112597;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1+ Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 1, None, None, None, None);
    // You can only control 1 "Worldsea Dragon Zealantis"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Banish all monsters on the field, then Special Summon as many monsters as possible that were banished by this effect, to their owners' fields, face-up, or in face-down Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmsptg)
    effect::set_target(d, e1, Some(cost_target(rmsptg)));
    // e1:SetOperation(s.rmspop)
    effect::set_operation(d, e1, Some(operation(rmspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy cards on the field up to the number of co-linked monsters on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Card.IsAbleToRemove` (no extra arguments)
fn is_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `function() return Duel.IsBattlePhase() end`
fn cond2(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
/// `s.rmsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&is_removable), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_REMOVED as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local owner=c:GetOwner()
    let owner = card::get_owner(d, c);
    // return c:IsFaceup() and c:IsLocation(LOCATION_REMOVED) and not c:IsReason(REASON_REDIRECT)
    //     and (c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,owner)
    //     or c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,owner))
    card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_REMOVED)
        && !card::is_reason(d, c, REASON_REDIRECT)
        && (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(owner), None)
            || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(owner), None))
}
/// `s.rmspop(e,tp,eg,ep,ev,re,r,rp)`
fn rmspop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&is_removable), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local sg=og:Filter(s.spfilter,nil,e,tp)
        let sg = group::filter(d, &og, &|d, c| spfilter(d, c, e, tp), NO_EX);
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local your_sg,opp_sg=sg:Split(Card.IsOwner,nil,tp)
        let (mut your_sg, mut opp_sg) = group::split(d, &sg, &|d, c| card::is_owner(d, c, tp), NO_EX);
        // local your_ft,opp_ft=Duel.GetLocationCount(tp,LOCATION_MZONE),Duel.GetLocationCount(1-tp,LOCATION_MZONE)
        let your_ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        let opp_ft = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
        // if #your_sg>your_ft then
        if your_sg.len() as i32 > your_ft {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // your_sg=your_sg:Select(tp,your_ft,your_ft,nil)
            your_sg = group::select(d, &your_sg, tp, your_ft as u16, your_ft as u16, false, NO_EX).nil();
        }
        // if #opp_sg>opp_ft then
        if opp_sg.len() as i32 > opp_ft {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // opp_sg=opp_sg:Select(tp,opp_ft,opp_ft,nil)
            opp_sg = group::select(d, &opp_sg, tp, opp_ft as u16, opp_ft as u16, false, NO_EX).nil();
        }
        // sg=your_sg+opp_sg
        let sg = group::union(&your_sg, &opp_sg);
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // local sump=0
            let mut sump: u8 = 0;
            // local owner=sc:GetOwner()
            let owner = card::get_owner(d, sc);
            // if sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,owner) then sump=sump|POS_FACEUP end
            if card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(owner), None) {
                sump |= POS_FACEUP as u8;
            }
            // if sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,owner) then sump=sump|POS_FACEDOWN_DEFENSE end
            if card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(owner), None) {
                sump |= POS_FACEDOWN_DEFENSE as u8;
            }
            // Duel.SpecialSummonStep(sc,0,tp,owner,false,false,sump)
            duel::special_summon_step(d, sc, 0, tp, owner, false, false, sump, None);
        }
        // local fdg=sg:Filter(Card.IsFacedown,nil)
        let fdg = group::filter(d, &sg, &|d, c| card::is_facedown(d, c), NO_EX);
        // if #fdg>0 then Duel.ConfirmCards(1-tp,fdg) end
        if !fdg.is_empty() {
            duel::confirm_cards(d, 1 - tp, fdg);
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.desctfilter(c)`
fn desctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetMutualLinkedGroupCount()>0
    card::get_mutual_linked_group_count(d, c) > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // if chk==0 then return #dg>0 and Duel.IsExistingMatchingCard(s.desctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return !dg.is_empty() && duel::is_existing_matching_card(d, Some(&desctfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(s.desctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&desctfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
