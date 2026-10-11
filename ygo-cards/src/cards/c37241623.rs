//! Heraldry Record
use super::prelude::*;

pub const ID: u32 = 37241623;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s[0]=nil
        set_s_var(d, ID, 0, Value::Nil);
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DETACH_MATERIAL)
        effect::set_code(d, ge1, EVENT_DETACH_MATERIAL);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, _a: &EffArgs) {
    // local cid=Duel.GetCurrentChain()
    let cid = duel::get_current_chain(d, false);
    // if cid>0 then
    if cid > 0 {
        // s[0]=Duel.GetChainInfo(cid,CHAININFO_CHAIN_ID)
        let v = duel::get_chain_info(d, cid as u8, &[CHAININFO_CHAIN_ID]).remove(0);
        set_s_var(d, ID, 0, v);
    }
}
// Lua `==` between two values (numbers or nil; `nil==nil` is true)
fn lua_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Int(x), Value::Int(y)) => x == y,
        _ => false,
    }
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)==s[0] and re:IsActiveType(TYPE_XYZ) and Duel.IsChainNegatable(ev)
    if a.rp == a.tp {
        return false;
    }
    let chain_id = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]).remove(0);
    if !lua_equal(&chain_id, &s_var(d, ID, 0)) {
        return false;
    }
    effect::is_active_type(d, a.re.nil(), TYPE_XYZ) && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, h, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let h = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, h, a.re.nil()) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, h, REASON_EFFECT, None, None);
        }
    }
}
