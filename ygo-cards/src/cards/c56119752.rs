//! Present Card
use super::prelude::*;

pub const ID: u32 = 56119752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW);
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
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(1-tp,5) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, 1 - a.tp, 5);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, ct);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,5)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 5);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)~=0 then
    if duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(1-tp,5,REASON_EFFECT)
        duel::draw(d, 1 - a.tp, 5, REASON_EFFECT);
    }
}
