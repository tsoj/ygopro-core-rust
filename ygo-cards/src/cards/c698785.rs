//! Thunder End Dragon
use super::prelude::*;

pub const ID: u32 = 698785;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_NORMAL),8,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, sc, st, Some(tp)))), Some(8), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // negate activate
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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
