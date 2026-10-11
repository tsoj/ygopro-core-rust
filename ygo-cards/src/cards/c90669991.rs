//! Pineapple Blast
use super::prelude::*;

pub const ID: u32 = 90669991;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    a.ep == a.tp && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)-Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) - duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), ct as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let ct = duel::get_field_group_count(d, tp, LOCATION_MZONE, 0);
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)-ct<=0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) - ct <= 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 1));
    // local dg=g:Select(1-tp,ct,ct,nil)
    let dg = group::select(d, &g, 1 - tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.HintSelection(dg,true)
    duel::hint_selection(d, dg.clone(), true);
    // Duel.Destroy(g-dg,REASON_EFFECT)
    let mut rest = g.clone();
    group::remove_card(&mut rest, &dg);
    duel::destroy(d, rest, REASON_EFFECT, None, None);
}
