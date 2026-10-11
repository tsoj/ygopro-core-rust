//! Success Probability 0%
use super::prelude::*;

pub const ID: u32 = 6859683;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsFacedown() or c:IsType(TYPE_FUSION)) and c:IsAbleToGrave()
    (card::is_facedown(d, c) || card::is_type(d, c, TYPE_FUSION)) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // if chk==0 then return g:FilterCount(s.filter,nil)>=2 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &filter, NO_EX) >= 2;
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsType,1-tp,LOCATION_EXTRA,0,nil,TYPE_FUSION)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_type(d, c, TYPE_FUSION)), 1 - tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local rg=g:RandomSelect(tp,2)
    let rg = group::random_select(d, &g, tp, 2);
    // Duel.SendtoGrave(rg,REASON_EFFECT)
    duel::send_to_grave(d, &rg, REASON_EFFECT, None, None);
}
