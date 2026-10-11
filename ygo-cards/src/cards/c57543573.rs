//! Worm Illidan
use super::prelude::*;

pub const ID: u32 = 57543573;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0xf], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0xf)
    card::enable_counter_permit(d, c, 0xf, Some(LOCATION_MZONE), None);
    // add counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_MSET)
    effect::set_code(d, e1, EVENT_MSET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.accon1)
    effect::set_condition(d, e1, Some(condition(accon1)));
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SSET)
    effect::set_code(d, e2, EVENT_SSET);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e3, EVENT_CHANGE_POS);
    // e3:SetCondition(s.accon2)
    effect::set_condition(d, e3, Some(condition(accon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetCondition(s.accon3)
    effect::set_condition(d, e4, Some(condition(accon3)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetCost(s.descost)
    effect::set_cost(d, e5, Some(cost_target(descost)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.filter2(c,tp)`
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and (c:GetPreviousPosition()&POS_FACEUP)~=0 and (c:GetPosition()&POS_FACEDOWN)~=0
    card::is_controler(d, c, tp)
        && (card::get_previous_position(d, c) & POS_FACEUP) != 0
        && (card::get_position(d, c) & POS_FACEDOWN) != 0
}
/// `s.filter3(c,tp)`
fn filter3(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsFacedown()
    card::is_controler(d, c, tp) && card::is_facedown(d, c)
}
/// `s.accon1(e,tp,eg,ep,ev,re,r,rp)`
fn accon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(tp)
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, a.tp)
}
/// `s.accon2(e,tp,eg,ep,ev,re,r,rp)`
fn accon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter2,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| filter2(d, c, tp), 1, NO_EX)
}
/// `s.accon3(e,tp,eg,ep,ev,re,r,rp)`
fn accon3(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter3,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| filter3(d, c, tp), 1, NO_EX)
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0xf,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0xf, 1, false);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0xf,2,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, a.tp, 0xf, 2, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,0xf,2,REASON_COST)
    card::remove_counter(d, h, a.tp, 0xf, 2, REASON_COST);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
