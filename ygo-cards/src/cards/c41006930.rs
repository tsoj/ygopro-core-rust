//! Strike Ninja
use super::prelude::*;

pub const ID: u32 = 41006930;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.rmcost)
    effect::set_cost(d, e1, Some(cost_target(rmcost)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,2,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,2,2,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 2, 2, false, h.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove(d, h, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.Remove(c,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)
        duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
