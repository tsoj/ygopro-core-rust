//! Blind Obliteration
use super::prelude::*;

pub const ID: u32 = 22802010;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevelBelow(1) or c:IsRankBelow(1))
    card::is_faceup(d, c) && (card::is_level_below(d, c, 1) || card::is_rank_below(d, c, 1))
}
// s.filter1(c)
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevelBelow(11) or c:IsRankBelow(11))
    card::is_faceup(d, c) && (card::is_level_below(d, c, 11) || card::is_rank_below(d, c, 11))
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter1), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// s.filter2(c,lv)
fn filter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and (c:IsLevelBelow(lv-1) or c:IsRankBelow(lv-1))
    card::is_faceup(d, c) && (card::is_level_below(d, c, lv - 1) || card::is_rank_below(d, c, lv - 1))
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local d1,d2=Duel.TossDice(tp,2)
    let dice = duel::toss_dice(d, tp, 2, 0);
    let (d1, d2) = (dice[0], dice[1]);
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,nil,d1+d2)
    let lv = (d1 + d2) as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, lv)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
