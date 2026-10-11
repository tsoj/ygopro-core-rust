//! Pendulum Storm
use super::prelude::*;

pub const ID: u32 = 83461421;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,LOCATION_PZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_PZONE, LOCATION_PZONE) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, LOCATION_PZONE);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,LOCATION_PZONE)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, LOCATION_PZONE);
    // if Duel.Destroy(g,REASON_EFFECT)~=0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        // local dg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_ONFIELD,nil)
        let dg = duel::get_matching_group(d, Some(&filter2), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            // local sg=dg:Select(tp,1,1,nil)
            let sg = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
    }
}
