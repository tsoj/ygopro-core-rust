//! Trap of Darkness
use super::prelude::*;

pub const ID: u32 = 79766336;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // copy trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DRAW_PHASE|TIMINGS_CHECK_MONSTER_E,TIMING_DRAW_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E, Some(TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<=3000
    duel::get_lp(d, a.tp) <= 3000
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and not c:IsCode(id) and c:IsAbleToRemove() and c:CheckActivateEffect(false,true,false)~=nil
    card::is_normal_trap(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_remove(d, c, None, None, None)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// The event values `CheckActivateEffect` returns after the effect (`ceg,cep,cev,cre,cr,crp`).
fn event_values(pe: Option<ygo_core::duel::Event>) -> [Value; 6] {
    match pe {
        Some(pe) => [
            pe.event_cards.clone().map_or(Value::Nil, Value::Group),
            Value::Int(pe.event_player as i64),
            Value::Int(pe.event_value as i64),
            pe.reason_effect.map_or(Value::Nil, Value::Effect),
            Value::Int(pe.reason as i64),
            Value::Int(pe.reason_player as i64),
        ],
        None => [Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil],
    }
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of the copied effect, with an optional extra value
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
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject()
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        // local tg=te:GetTarget()
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        return match effect::get_target(d, te) {
            Some(tg) => {
                let args = copy_args(a, &[Value::Int(0), Value::Card(chkc)]);
                tg.call(d, &args).truthy()
            }
            None => false,
        };
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &g).nil();
    // local te,ceg,cep,cev,cre,cr,crp=g:GetFirst():CheckActivateEffect(false,true,true)
    let (te, pe) = card::check_activate_effect(d, gc, false, true, true).nil();
    let ev = event_values(pe);
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // g:GetFirst():CreateEffectRelation(e)
    card::create_effect_relation(d, gc, a.e);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = tg {
        let args = copy_args_with_event(a, &ev, Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, e_lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `(e,tp,ceg,cep,cev,cre,cr,crp,chk)`: the copied target gets the event values of the trap
fn copy_args_with_event(a: &EffArgs, ev: &[Value; 6], chk: Value) -> Vec<Value> {
    let mut v = vec![Value::Effect(a.e), Value::Int(a.tp as i64)];
    v.extend_from_slice(ev);
    v.push(chk);
    v
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = match effect::get_label_object(d, a.e) {
        Value::Effect(te) => te,
        _ => return,
    };
    // if not te then return end
    // if not te:GetHandler():IsRelateToEffect(e) then return end
    let th = effect::get_handler(d, te).nil();
    if !card::is_relate_to_effect(d, th, a.e) {
        return;
    }
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, te_lo);
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &copy_args(a, &[]));
    }
    // Duel.Remove(te:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let th = effect::get_handler(d, te).nil();
    duel::remove(d, th, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
