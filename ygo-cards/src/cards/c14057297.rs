//! Multiple Destruction
use super::prelude::*;

pub const ID: u32 = 14057297;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>=3 and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=3
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) >= 3 && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) >= 3
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,5) and Duel.IsPlayerCanDraw(1-tp,5) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 5) && duel::is_player_can_draw(d, 1 - a.tp, 5);
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, LOCATION_HAND);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,5)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 5);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,LOCATION_HAND)==0 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, LOCATION_HAND) == 0 {
        return;
    }
    // local p=tp
    let mut p = a.tp;
    // local st=0
    let mut st: i32 = 0;
    // for i=1,2 do
    for _ in 0..2 {
        // local sg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
        let sg = duel::get_field_group(d, p, LOCATION_HAND, 0);
        // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let ct = group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX) as i32;
        // if ct>0 then
        if ct > 0 {
            // st=st+ct
            st += ct;
            // Duel.SortDeckbottom(p,p,ct)
            duel::sort_deckbottom(d, p, p, ct as u16);
        }
        // p=1-p
        p = 1 - p;
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // Duel.SetLP(tp,lp-st*300)
    duel::set_lp(d, a.tp, lp - st * 300);
    // if Duel.GetLP(tp)>0 then
    if duel::get_lp(d, a.tp) > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,5,REASON_EFFECT)
        duel::draw(d, a.tp, 5, REASON_EFFECT);
        // Duel.Draw(1-tp,5,REASON_EFFECT)
        duel::draw(d, 1 - a.tp, 5, REASON_EFFECT);
    }
}
