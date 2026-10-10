//! Labrynth Barrage
use super::prelude::*;

pub const ID: u32 = 68779682;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and not rc:IsStatus(STATUS_ACT_FROM_HAND)
    //     and rc:IsPreviousPosition(POS_FACEDOWN) and rp==tp
    //     and rc:IsNormalTrap() and not rc:IsCode(id)
    //     and rc:CheckActivateEffect(false,true,true)~=nil
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && !card::is_status(d, rc, STATUS_ACT_FROM_HAND)
        && card::is_previous_position(d, rc, POS_FACEDOWN)
        && a.rp == a.tp
        && card::is_normal_trap(d, rc)
        && !card::is_code(d, rc, &[ID])
        && card::check_activate_effect(d, rc, false, true, true).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return true
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local te,ceg,cep,cev,cre,cr,crp=re:GetHandler():CheckActivateEffect(false,true,true)
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let (te, info) = card::check_activate_effect(d, rc, false, true, true).nil();
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // local tg=te:GetTarget()
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = effect::get_target(d, te) {
        let mut args = vec![Value::Effect(a.e), Value::Int(a.tp as i64)];
        match info {
            Some(ev) => args.extend([
                ev.event_cards.clone().map_or(Value::Nil, Value::Group),
                Value::Int(ev.event_player as i64),
                Value::Int(ev.event_value as i64),
                Value::effect(ev.reason_effect),
                Value::Int(ev.reason as i64),
                Value::Int(ev.reason_player as i64),
            ]),
            None => args.extend([Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil]),
        }
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// The arguments `e,tp,eg,ep,ev,re,r,rp` of `a`, as a Lua call passes them.
fn call_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e);
    // if not te then return end
    let Some(te) = te.as_effect() else { return };
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &call_args(a));
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local ct=Duel.IsTurnPlayer(tp) and 2 or 3
    let ct: u16 = if duel::is_turn_player(d, a.tp) { 2 } else { 3 };
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    // --no damage
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // e1:SetReset(RESET_PHASE|PHASE_END,ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, ct);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // e2:SetReset(RESET_PHASE|PHASE_END,ct)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, ct);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    let rp = a.int(4) as u8;
    // if rp==e:GetHandlerPlayer() and (r&REASON_EFFECT)~=0 then return 0
    if rp == effect::get_handler_player(d, e) && (r & REASON_EFFECT) != 0 {
        0
    // else return val end
    } else {
        val
    }
}
