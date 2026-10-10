//! Jowgen the Spiritualist
use super::prelude::*;

pub const ID: u32 = 41855169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`: `return c:IsDiscardable() and c:IsAbleToGraveAsCost()`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // if chk==0 then return #g>0 and g:FilterCount(s.cfilter,nil)==#g end
    if a.chk == Some(0) {
        return !g.is_empty() && group::filter_count(d, &g, &cfilter, NO_EX) == g.len();
    }
    // local sg=g:RandomSelect(tp,1)
    let sg = group::random_select(d, &g, a.tp, 1);
    // Duel.SendtoGrave(sg,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, sg, REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `Card.IsSummonType(c,SUMMON_TYPE_SPECIAL)` as a filter
fn special_summoned(d: &mut Duel, c: CardId) -> bool {
    card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSummonType,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,SUMMON_TYPE_SPECIAL) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&special_summoned), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::get_matching_group(d, Some(&special_summoned), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,LOCATION_MZONE,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::get_matching_group(d, Some(&special_summoned), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
