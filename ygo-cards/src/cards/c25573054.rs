//! Transcendent Wings
use super::prelude::*;

pub const ID: u32 = 25573054;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[57116033, 98585345], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(57116033) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[57116033]) && card::is_able_to_grave_as_cost(d, c)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(98585345) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[98585345]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,e,tp)
    if !aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0 {
        return (false, false);
    }
    let e = e.nil();
    (group::is_exists(d, sg, &|d, c| chk(d, c, sg, e, tp), 1, NO_EX), false)
}
// function s.chk(c,sg,e,tp)
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(57116033) and c:IsLocation(LOCATION_ONFIELD) and sg:IsExists(Card.IsLocation,2,c,LOCATION_HAND)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,sg,e,tp)
    card::is_code(d, c, &[57116033])
        && card::is_location(d, c, LOCATION_ONFIELD)
        && group::is_exists(d, sg, &|d, x| card::is_location(d, x, LOCATION_HAND), 2, c.into())
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, sg.into())
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(1)
    effect::set_label(d, e, &[1]);
    // local hg=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let hg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, h.into());
    // local mg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let mg = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_ONFIELD, 0, h.into());
    // local g=hg:Clone()
    let mut g = hg.clone();
    // g:Merge(mg)
    group::add_card(&mut g, &mg);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>-1 and #hg>1 and #mg>0 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return ft > -1
            && hg.len() > 1
            && !mg.is_empty()
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==1 then
        if effect::get_label(d, e) == 1 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // return true
            return true;
        }
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
