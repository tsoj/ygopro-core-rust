//! Dual Avatar Compact
//! 双天の使命
use super::prelude::*;

pub const ID: u32 = 75157704;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_DUAL_AVATAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Copy effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_DUAL_AVATAR) and not c:IsCode(id)
    //     and c:IsAbleToRemoveAsCost() and c:CheckActivateEffect(false,true,false)~=nil
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g).map_or(Value::Nil, Value::Card);
    effect::set_label_object(d, a.e, first);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    //     local res=e:GetLabel()~=0
    //     e:SetLabel(0)
    //     return res
    // end
    if a.chk == Some(0) {
        let res = effect::get_label(d, a.e) != 0;
        effect::set_label(d, a.e, &[0]);
        return res;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local te,ceg,cep,cev,cre,cr,crp=tc:CheckActivateEffect(false,true,true)
    let (te_opt, pe) = match card::check_activate_effect(d, tc, false, true, true) {
        Some((te, pe)) => (Some(te), pe),
        None => (None, None),
    };
    let ev = event_values(pe);
    // tc:CreateEffectRelation(e)
    card::create_effect_relation(d, tc, a.e);
    // local tg=te:GetTarget()
    let te = te_opt.nil();
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = tg {
        let _ = tg.call(d, &target_args(a, &ev, 1));
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
/// `(e,tp,ceg,cep,cev,cre,cr,crp,chk)` for the copied target
fn target_args(a: &EffArgs, ev: &[Value; 6], chk: i64) -> Vec<Value> {
    let mut v = vec![Value::Effect(a.e), Value::Int(a.tp as i64)];
    v.extend_from_slice(ev);
    v.push(Value::Int(chk));
    v
}
/// `(e,tp,eg,ep,ev,re,r,rp)` for the copied operation
fn op_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    // if not te then return end
    let Value::Effect(te) = effect::get_label_object(d, a.e) else {
        return;
    };
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = op {
        let _ = op.call(d, &op_args(a));
    }
}
