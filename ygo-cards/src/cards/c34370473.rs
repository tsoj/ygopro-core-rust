//! Gryphon's Feather Duster
//! グリフォンの羽根帚
use super::prelude::*;

pub const ID: u32 = 34370473;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_RECOVER);
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
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,c)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, 0, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = group::get_count(&g);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, (n * 500) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, 0, h.into());
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // Duel.Recover(tp,ct*500,REASON_EFFECT)
    duel::recover(d, tp, ct as i64 * 500, REASON_EFFECT, false, None);
}
