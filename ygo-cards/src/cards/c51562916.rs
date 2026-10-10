//! Big Wave Small Wave
use super::prelude::*;

pub const ID: u32 = 51562916;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let mut ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ct==0 or ft<=0 then return end
    if ct == 0 || ft <= 0 {
        return;
    }
    // ct=math.min(ct,ft)
    ct = ct.min(ft);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=math.min(ct,1) end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = ct.min(1);
    }
    // Duel.RaiseEvent(Group.CreateGroup(),EVENT_ADJUST,nil,0,PLAYER_NONE,PLAYER_NONE,0)
    duel::raise_event(d, CardSet::new(), EVENT_ADJUST, None, 0, PLAYER_NONE as u8, PLAYER_NONE as u8, 0);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #sg~=0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local spg=sg:Select(tp,1,ct,nil)
        let spg = group::select(d, &sg, tp, 1, ct as u16, false, NO_EX).nil();
        // Duel.SpecialSummon(spg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, spg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
