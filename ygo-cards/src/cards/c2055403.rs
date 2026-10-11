//! Time-Space Trap Hole
use super::prelude::*;

pub const ID: u32 = 2055403;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not c:IsSummonPlayer(tp) and c:GetSummonLocation()&(LOCATION_HAND|LOCATION_EXTRA)~=0
    //   and c:IsAbleToDeck() and c:IsLocation(LOCATION_MZONE)
    !card::is_summon_player(d, c, tp)
        && card::get_summon_location(d, c) & (LOCATION_HAND | LOCATION_EXTRA) != 0
        && card::is_able_to_deck(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // local g=eg:Filter(s.filter,nil,tp)
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    // local ct=#g
    let ct = g.len();
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, &eg);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), ct as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter,nil,tp):Filter(Card.IsRelateToEffect,nil,e)
    let eg = a.eg();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    let g = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SetLP(tp,Duel.GetLP(tp)-ct*1000)
            let lp = duel::get_lp(d, a.tp);
            duel::set_lp(d, a.tp, lp - ct as i32 * 1000);
        }
    }
}
