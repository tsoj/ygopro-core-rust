//! Curse Reflection Doll
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 33391067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reflect effect change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself when you take battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.setcond)
    effect::set_condition(d, e2, Some(condition(setcond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and aux.damcon1(e,tp,eg,ep,ev,re,r,rp)
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let eg = a.eg.clone().map_or(Value::Nil, Value::Group);
    aux::damcon1()
        .call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), eg, Value::Int(a.ep as i64), Value::Int(a.ev as i64), Value::Effect(re), Value::Int(a.r as i64), Value::Int(a.rp as i64)])
        .truthy()
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // e1:SetValue(s.refcon)
    effect::set_value_fn(d, e1, lua_fn(refcon));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.refcon(e,re,val,r,rp,rc)`
fn refcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if cc == 0 || ((a.int(3) as u32) & REASON_EFFECT) == 0 {
        return false;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    // return cid==e:GetLabel()
    cid == effect::get_label(d, e)
}
/// `s.setcond(e,tp,eg,ep,ev,re,r,rp)`
fn setcond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp
    a.ep == a.tp
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, tp, c, None, true);
    }
}
