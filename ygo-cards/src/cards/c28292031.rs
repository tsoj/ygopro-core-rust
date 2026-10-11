//! Dinomorphia Reversion
//! ダイノルフィア・リヴァージョン
use super::prelude::*;

pub const ID: u32 = 28292031;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMORPHIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e1a=e1:Clone()
    let e1a = effect::clone(d, e1);
    // e1a:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1a, EVENT_SUMMON);
    // e1a:SetCondition(s.condition2)
    effect::set_condition(d, e1a, Some(condition(condition2)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1:Clone()
    let e1b = effect::clone(d, e1);
    // e1b:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON);
    // e1b:SetCondition(s.condition2)
    effect::set_condition(d, e1b, Some(condition(condition2)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1:Clone()
    let e1c = effect::clone(d, e1);
    // e1c:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1c, EVENT_SPSUMMON);
    // e1c:SetCondition(s.condition2)
    effect::set_condition(d, e1c, Some(condition(condition2)));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // No battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.nodamcon)
    effect::set_condition(d, e2, Some(condition(nodamcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(s.nodamop)
    effect::set_operation(d, e2, Some(operation(nodamop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMORPHIA) and c:IsType(TYPE_FUSION)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMORPHIA as u16]) && card::is_type(d, c, TYPE_FUSION)
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_current_chain(d, true) == 0 && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.effilter(c)`
fn effilter(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsAbleToRemoveAsCost() or not c:IsType(TYPE_COUNTER) then return false end
    if !card::is_able_to_remove_as_cost(d, c, None) || !card::is_type(d, c, TYPE_COUNTER) {
        return false;
    }
    // local eff=c:CheckActivateEffect(false,true,false)
    // return eff and eff:GetOperation()~=nil
    match card::check_activate_effect(d, c, false, true, false) {
        Some((eff, _)) => effect::get_operation(d, eff).is_some(),
        None => false,
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.effilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&effilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let half = duel::get_lp(d, tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, tp, half);
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of a copied effect, followed by `tail`
fn copy_args(a: &EffArgs, tail: &[Value]) -> Vec<Value> {
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
    v.extend_from_slice(tail);
    v
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject()
        // if not te then return end
        let Value::Effect(te) = effect::get_label_object(d, e) else { return false };
        // local tg=te:GetTarget()
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        return match effect::get_target(d, te) {
            Some(tg) => tg.call(d, &copy_args(a, &[Value::Int(0), Value::Card(chkc)])).truthy(),
            None => false,
        };
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.effilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&effilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if Duel.Remove(g,POS_FACEUP,REASON_COST)==0 then return end
    if duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None) == 0 {
        return false;
    }
    // local te,ceg,cep,cev,cre,cr,crp=g:GetFirst():CheckActivateEffect(false,true,true)
    let gc = group::get_first(d, &g).nil();
    let (te, ev) = card::check_activate_effect(d, gc, false, true, true).nil();
    // (the event values: nil where the core has none)
    let pe = ev.unwrap_or_default();
    let ceg = pe.event_cards.clone().map_or(Value::Nil, Value::Group);
    let cre = pe.reason_effect.map_or(Value::Nil, Value::Effect);
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_label(d, te);
    effect::set_label(d, e, &[te_label]);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, te_lo);
    // local tg=te:GetTarget()
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = effect::get_target(d, te) {
        let args = vec![
            Value::Effect(e),
            Value::Int(tp as i64),
            ceg,
            Value::Int(pe.event_player as i64),
            Value::Int(pe.event_value as i64),
            cre,
            Value::Int(pe.reason as i64),
            Value::Int(pe.reason_player as i64),
            Value::Int(1),
        ];
        tg.call(d, &args);
    }
    // te:SetLabel(e:GetLabel())
    let e_label = effect::get_label(d, e);
    effect::set_label(d, te, &[e_label]);
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, e);
    effect::set_label_object(d, te, e_lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    // e:SetOperation(s.activate)
    effect::set_operation(d, e, Some(operation(activate)));
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local te=e:GetLabelObject()
    // if not te then return end
    let Value::Effect(te) = effect::get_label_object(d, e) else { return };
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_label(d, te);
    effect::set_label(d, e, &[te_label]);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, te_lo);
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &copy_args(a, &[]));
    }
    // te:SetLabel(e:GetLabel())
    let e_label = effect::get_label(d, e);
    effect::set_label(d, te, &[e_label]);
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, e);
    effect::set_label_object(d, te, e_lo);
    // e:SetOperation(nil)
    effect::set_operation(d, e, None);
}
/// `s.nodamcon(e,tp,eg,ep,ev,re,r,rp)`
fn nodamcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<=2000 and Duel.GetBattleDamage(tp)>0 and not Duel.IsPlayerAffectedByEffect(tp,EFFECT_AVOID_BATTLE_DAMAGE)
    duel::get_lp(d, a.tp) <= 2000
        && duel::get_battle_damage(d, a.tp) > 0
        && duel::is_player_affected_by_effect(d, a.tp, EFFECT_AVOID_BATTLE_DAMAGE).is_none()
}
/// `s.nodamop(e,tp,eg,ep,ev,re,r,rp)`
fn nodamop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,EFFECT_AVOID_BATTLE_DAMAGE) then return end
    if duel::is_player_affected_by_effect(d, tp, EFFECT_AVOID_BATTLE_DAMAGE).is_some() {
        return;
    }
    // No battle damage
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
