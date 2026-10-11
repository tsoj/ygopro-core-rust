//! Gungnir, Dragon of the Ice Barrier
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 65749035;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_WATER),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
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
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGraveAsCost()
    card::is_able_to_grave_as_cost(d, c)
}
/// The arguments of the Lua cost call `aux.IceBarrierDiscardCost(...)(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_args(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        Value::Nil,
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local rt=math.min(2,Duel.GetTargetCount(aux.TRUE,tp,0,LOCATION_ONFIELD,nil))
    let rt = duel::get_target_count(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX).min(2);
    // aux.IceBarrierDiscardCost(s.costfilter,true,1,rt)
    let f = aux::ice_barrier_discard_cost(Some(Arc::new(|d: &mut Duel, c: CardId| costfilter(d, c))), true, Some(1), Some(rt));
    // if chk==0 then return aux.IceBarrierDiscardCost(s.costfilter,true,1,rt)(e,tp,eg,ep,ev,re,r,rp,0) end
    if a.chk == Some(0) {
        return f.call(d, &cost_args(a, 0)).truthy();
    }
    // e:SetLabel(aux.IceBarrierDiscardCost(s.costfilter,true,1,rt)(e,tp,eg,ep,ev,re,r,rp,1))
    let cg = f.call(d, &cost_args(a, 1));
    effect::set_label(d, a.e, &[cg.int()]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local eg=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,ct,ct,nil)
    let eg = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), ct as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local rg=Duel.GetTargetCards(e)
    let rg = duel::get_target_cards(d, a.e).nil();
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Destroy(rg,REASON_EFFECT)
        duel::destroy(d, &rg, REASON_EFFECT, None, None);
    }
}
