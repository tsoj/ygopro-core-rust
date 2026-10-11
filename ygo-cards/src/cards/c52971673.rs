//! Token Sundae
use super::prelude::*;

pub const ID: u32 = 52971673;

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
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN)
}
// function s.dfilter(c)
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOKEN)
    !card::is_type(d, c, TYPE_TOKEN)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&dfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local dt=Duel.Destroy(g,REASON_EFFECT)
    let dt = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if dt==0 then return end
    if dt == 0 {
        return;
    }
    // local dg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=dg:Select(tp,1,dt,nil)
        let sg = group::select(d, &dg, tp, 1, dt as u16, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, &sg, true);
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
