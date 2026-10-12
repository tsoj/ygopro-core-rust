//! Trirealm Rift Gospel
//! 異解△福音
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100458040;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this card by banishing (face-down) 5 cards from your GY and/or the top of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, you can Normal Summon 1 "Trirealm Rift" monster, in addition to your Normal Summon/Set (you can only gain this effect once per turn)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SUMMON_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return c:IsSetCard(SET_XENOVADER) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_XENOVADER as u16]))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During your Main Phase: You can banish (face-down) 1 "Trirealm Rift" monster you control whose original Level is 4 or lower, and if you do, Special Summon 1 of your face-down banished Level 5 or higher "Trirealm Rift" monsters. You can only use this effect of "Trirealm Rift Gospel" once per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.bansptg)
    effect::set_target(d, e3, Some(cost_target(bansptg)));
    // e3:SetOperation(s.banspop)
    effect::set_operation(d, e3, Some(operation(banspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetDecktopGroup(tp,5)
    let dg = duel::get_decktop_group(d, a.tp, 5);
    // local dc=dg:FilterCount(Card.IsAbleToRemoveAsCost,nil,POS_FACEDOWN)
    let dc = group::filter_count(d, &dg, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), NO_EX) as i64;
    // local gyct=Duel.GetMatchingGroupCount(Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,nil,POS_FACEDOWN)
    let gyct = duel::get_matching_group_count(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // if chk==0 then return dc==5 or gyct+dc>=5 end
    if a.chk == Some(0) {
        return dc == 5 || gyct + dc >= 5;
    }
    // local rg=Group.CreateGroup()
    let mut rg = group::create_group(&[]);
    // if dc<5 or (gyct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
    if dc < 5 || (gyct > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // rg=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,math.max(5-dc,1),5,nil,POS_FACEDOWN)
        rg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))), a.tp, LOCATION_GRAVE, 0, (5 - dc).max(1) as u16, 5, false, NO_EX).nil();
        // Duel.HintSelection(rg)
        duel::hint_selection(d, rg.clone(), true);
    }
    // if #rg<5 then
    if rg.len() < 5 {
        // rg=rg+Duel.GetDecktopGroup(tp,5-#rg)
        let deck = duel::get_decktop_group(d, a.tp, (5 - rg.len()) as u32);
        rg = group::union(&rg, &deck);
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(rg,POS_FACEDOWN,REASON_COST)
    duel::remove(d, rg, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}

/// `s.banfilter(c,tp)`
fn banfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_XENOVADER) and c:GetOriginalLevel()<=4 and c:IsAbleToRemove(tp,POS_FACEDOWN) and c:IsFaceup()
    //     and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_XENOVADER as u16])
        && card::get_original_level(d, c) <= 4
        && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}

/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and c:IsLevelAbove(5)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_facedown(d, c)
        && card::is_set_card(d, c, &[SET_XENOVADER as u16])
        && card::is_level_above(d, c, 5)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// `s.bansptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bansptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.banfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| banfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}

/// `s.banspop(e,tp,eg,ep,ev,re,r,rp)`
fn banspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.banfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| banfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // if Duel.Remove(sc,POS_FACEDOWN,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_REMOVED) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::remove(d, sc, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, sc, LOCATION_REMOVED)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
