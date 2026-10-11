//! 霊獣の相絆 / Ritual Beast's Bond
use super::prelude::*;

pub const ID: u32 = 75457624;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RITUAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local bg = aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE)
    let bg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(rs),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(bg,POS_FACEUP,REASON_COST)
    duel::remove(d, bg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg)
    let e = e.nil();
    (
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX),
        false,
    )
}
/// `s.spfilter(c,e,tp,sg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: Option<&CardSet>) -> bool {
    // return c:IsSetCard(SET_RITUAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,true,false) and Duel.GetLocationCountFromEx(tp,tp,sg,c) > 0
    card::is_set_card(d, c, &[SET_RITUAL_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), sg.map_or(NO_EX, |g| g.into()), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
