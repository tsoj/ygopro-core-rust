//! System Down
use super::prelude::*;

pub const ID: u32 = 18895832;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_MACHINE) and aux.SpElimFilter(c,true,true)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_MACHINE) && aux::sp_elim_filter(d, c, true, true)
}
/// `s.tfilter(c)`
fn tfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsAbleToRemove()
    !card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE as u32 | LOCATION_GRAVE as u32, NO_EX);
        // return #g>0 and not g:IsExists(s.tfilter,1,nil)
        return !g.is_empty() && !group::is_exists(d, &g, &tfilter, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE as u32 | LOCATION_GRAVE as u32, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE as u32 | LOCATION_GRAVE as u32, NO_EX);
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
