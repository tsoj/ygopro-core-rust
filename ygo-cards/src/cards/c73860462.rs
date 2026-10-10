//! Wonder Xyz
use super::prelude::*;

pub const ID: u32 = 73860462;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // perform an xyz summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.xyztg)
    effect::set_target(d, e1, Some(cost_target(xyztg)));
    // e1:SetOperation(s.xyzop)
    effect::set_operation(d, e1, Some(operation(xyzop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Card.IsXyzSummonable` as a filter (no extra arguments)
fn xyz_summonable(d: &mut Duel, c: CardId) -> bool {
    card::is_xyz_summonable(d, c, None, None, 0, 0)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&xyz_summonable), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&xyz_summonable), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local tg=g:Select(tp,1,1,nil)
        let tg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.XyzSummon(tp,tg:GetFirst())
        let first = group::get_first(d, &tg).nil();
        duel::xyz_summon(d, a.tp, first, None, None, 0, 0);
    }
}
