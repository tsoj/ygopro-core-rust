//! Introduction to Gallantry
//! 義賊の入門書
use super::prelude::*;

pub const ID: u32 = 69091732;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(0,TIMING_TOHAND)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>4
    duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 4
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local g=Duel.GetFieldGroup(p,LOCATION_HAND,0)
    let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
    // local dg=g:RandomSelect(tp,1)
    let dg = group::random_select(d, &g, a.tp, 1);
    // Duel.SendtoGrave(dg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
}
