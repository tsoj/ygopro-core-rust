//! Hieratic Sun Dragon Overlord of Heliopolis
use super::prelude::*;

pub const ID: u32 = 64332231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupEx(tp,Card.IsReleasableByEffect,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_ex(d, a.tp, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), 1, duel::ReleaseArgs::default(), NO_EX)
            && duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local ct1=Duel.GetMatchingGroupCount(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let ct1 = duel::get_matching_group_count(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as u16;
    // local rg=Duel.SelectReleaseGroupEx(tp,Card.IsReleasableByEffect,1,ct1,nil)
    let rg = duel::select_release_group_ex(d, a.tp, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), 1, ct1, duel::ReleaseArgs::default(), NO_EX);
    // if rg then
    if let Some(rg) = rg {
        // local ct2=Duel.Release(rg,REASON_EFFECT)
        let ct2 = duel::release(d, rg, REASON_EFFECT, None);
        // if ct2==0 then return end
        if ct2 == 0 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct2,ct2,nil)
        let dg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct2 as u16, ct2 as u16, false, NO_EX).nil();
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
