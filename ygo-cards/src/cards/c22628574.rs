//! Fake Feather
use super::prelude::*;

pub const ID: u32 = 22628574;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Copy the Effects of a Normal Trap card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E, Some(TIMING_DRAW_PHASE | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_normal_trap(d, c) && !card::is_code(d, c, &[ID]) && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of this effect.
fn own_args(a: &EffArgs) -> Vec<Value> {
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        // local te=e:GetLabelObject()
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        // local tg=te:GetTarget()
        let Some(tg) = effect::get_target(d, te) else {
            return false;
        };
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,1,true)
        let mut args = own_args(a);
        args.push(Value::Int(1));
        args.push(Value::Bool(true));
        return tg.call(d, &args).truthy();
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_GRAVE,1,1,nil)
    let Some(g) = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX) else {
        return false;
    };
    // local te,eg,ep,ev,re,r,rp=g:GetFirst():CheckActivateEffect(false,true,true)
    let tc = group::get_first(d, &g).nil();
    let (te, info) = card::check_activate_effect(d, tc, false, true, true).nil();
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p0, p1) = effect::get_property(d, te);
    effect::set_property(d, a.e, p0, p1);
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = tg {
        let [eg, ep, ev, re, r, rp] = event_values(info);
        let args = vec![Value::Effect(a.e), Value::Int(tp as i64), eg, ep, ev, re, r, rp, Value::Int(1)];
        tg.call(d, &args);
    }
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
fn event_values(info: Option<ygo_core::duel::Event>) -> [Value; 6] {
    match info {
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
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let Some(te) = effect::get_label_object(d, a.e).as_effect() else {
        return;
    };
    // local op=te:GetOperation()
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &own_args(a));
    }
}
