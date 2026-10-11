//! Token Feastevil
use super::prelude::*;

pub const ID: u32 = 83675475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
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
    // return eg:IsExists(Card.IsType,1,nil,TYPE_TOKEN)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_type(d, c, TYPE_TOKEN), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_TOKEN)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (n * 300) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_TOKEN)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // Duel.Damage(1-tp,ct*300,REASON_EFFECT)
    duel::damage(d, 1 - tp, ct as i64 * 300, REASON_EFFECT, false, None);
}
