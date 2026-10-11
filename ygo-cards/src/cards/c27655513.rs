//! Skreech
use super::prelude::*;

pub const ID: u32 = 27655513;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to grave
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_BATTLE)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToGrave()
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
