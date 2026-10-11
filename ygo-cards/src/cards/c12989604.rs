//! World Legacy Awakens
use super::prelude::*;

pub const ID: u32 = 12989604;

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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_link_summonable(d, c, None, None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsLinkSummonable,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_link_summonable(d, c, None, None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.LinkSummon(tp,tc)
        duel::link_summon(d, a.tp, tc, None, None, 0, 0);
    }
}
