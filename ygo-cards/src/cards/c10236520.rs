//! Goe Goe the Gallant Ninja
use super::prelude::*;

pub const ID: u32 = 10236520;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // handes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>=5
    a.ep != a.tp && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) >= 5
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,2)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 2);
    true
}
// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(ep,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    // if #g<5 then return end
    if g.len() < 5 {
        return;
    }
    // local sg=g:RandomSelect(1-tp,2)
    let sg = group::random_select(d, &g, 1 - a.tp, 2);
    // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
}
