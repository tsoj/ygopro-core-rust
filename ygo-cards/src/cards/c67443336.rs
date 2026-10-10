//! Balance of Judgment
use super::prelude::*;

pub const ID: u32 = 67443336;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
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
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local t=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let t = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // local s=Duel.GetFieldGroupCount(tp,LOCATION_HAND|LOCATION_ONFIELD,0)
    let s = duel::get_field_group_count(d, tp, LOCATION_HAND | LOCATION_ONFIELD, 0);
    // return t>s
    t > s
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local t=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let t = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    // local s=Duel.GetFieldGroupCount(tp,LOCATION_HAND|LOCATION_ONFIELD,0)
    let s = duel::get_field_group_count(d, tp, LOCATION_HAND | LOCATION_ONFIELD, 0);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,t-s) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, (t - s) as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(t-s)
    duel::set_target_param(d, (t - s) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,t-s)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, t - s);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local t=Duel.GetFieldGroupCount(p,0,LOCATION_ONFIELD)
    let t = duel::get_field_group_count(d, p, 0, LOCATION_ONFIELD);
    // local s=Duel.GetFieldGroupCount(p,LOCATION_HAND|LOCATION_ONFIELD,0)
    let s = duel::get_field_group_count(d, p, LOCATION_HAND | LOCATION_ONFIELD, 0);
    // if t>s then
    if t > s {
        // Duel.Draw(p,t-s,REASON_EFFECT)
        duel::draw(d, p, (t - s) as u32, REASON_EFFECT);
    }
}
