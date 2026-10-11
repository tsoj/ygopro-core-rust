//! Kunoichi
use super::prelude::*;

pub const ID: u32 = 56514812;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_CONTROL_CHANGED)
    effect::set_code(d, e1, EVENT_CONTROL_CHANGED);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsStatus(STATUS_CHAINING) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_status(d, c, STATUS_CHAINING);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,ep,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.ep, 1);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(ep,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:RandomSelect(ep,1)
    let sg = group::random_select(d, &g, a.ep, 1);
    // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
    duel::send_to_grave(d, &sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
