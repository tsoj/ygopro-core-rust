//! Yosenju Wind Worship
use super::prelude::*;

pub const ID: u32 = 54880296;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YOSENJU],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return as many "Yosenju" Monster Cards you control to the hand as possible
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.yosenjufilter(c)`
fn yosenjufilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YOSENJU) and c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER)
    card::is_set_card(d, c, &[SET_YOSENJU as u16]) && card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.yosenjufilter,tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&yosenjufilter), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>=3
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.yosenjufilter,tp,LOCATION_ONFIELD,0,nil):Match(Card.IsAbleToHand,nil)
    let mut g = duel::get_matching_group(d, Some(&yosenjufilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.yosenjufilter,tp,LOCATION_ONFIELD,0,nil):Match(Card.IsAbleToHand,nil)
    let mut g = duel::get_matching_group(d, Some(&yosenjufilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) <= 0 {
        return;
    }
    if !group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local ct=5-Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
    let ct = 5 - duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    // if ct>0 and Duel.IsPlayerCanDraw(tp,ct) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if ct > 0 && duel::is_player_can_draw(d, tp, ct as u32) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, tp, ct as u32, REASON_EFFECT);
    }
}
