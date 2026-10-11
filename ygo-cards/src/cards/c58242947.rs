//! Junk Collector
use super::prelude::*;

pub const ID: u32 = 58242947;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // copy trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_DRAW_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsAbleToRemoveAsCost() and c:CheckActivateEffect(false,true,false)~=nil
    card::is_normal_trap(d, c) && card::is_able_to_remove_as_cost(d, c, Some(POS_FACEUP as u8)) && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, Some(POS_FACEUP as u8)) && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let mut g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local te=g:GetFirst():CheckActivateEffect(false,true,true)
    let first = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, first, false, true, true).map(|x| x.0);
    // s[Duel.GetCurrentChain()]=te
    let chain = duel::get_current_chain(d, false) as u64;
    set_s_var(d, ID, chain, te.map_or(Value::Nil, Value::Effect));
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// The arguments a copied effect's Lua function gets: `(e,tp,eg,ep,ev,re,r,rp, ...)`, `e` being this card's effect.
fn copy_args(a: &EffArgs, extra: &[Value]) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    v.extend_from_slice(extra);
    v
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local te=s[Duel.GetCurrentChain()]
    let chain = duel::get_current_chain(d, false) as u64;
    let te = s_var(d, ID, chain).as_effect();
    // if chkc then
    if a.chkc().is_some() {
        // local tg=te:GetTarget()
        let te = te.nil();
        let tg = effect::get_target(d, te).unwrap_or_else(|| crate::lua_error("attempt to call a nil value"));
        // return tg(e,tp,eg,ep,ev,re,r,rp,0,true)
        return tg.call(d, &copy_args(a, &[Value::Int(0), Value::Bool(true)])).truthy();
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if not te then return end
    let Some(te) = te else { return false };
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // local tg=te:GetTarget()
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = effect::get_target(d, te) {
        tg.call(d, &copy_args(a, &[Value::Int(1)]));
    }
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local te=s[Duel.GetCurrentChain()]
    let chain = duel::get_current_chain(d, false) as u64;
    // if not te then return end
    let Some(te) = s_var(d, ID, chain).as_effect() else { return };
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &copy_args(a, &[]));
    }
}
