//! Transaction Rollback
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 6351147;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Copy the activation effect of a Normal Trap from your opponent's GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DRAW_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DRAW_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E, None);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Copy the activation effect of a Normal Trap from your GY
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.gycost)
    effect::set_cost(d, e2, Some(cost_target(gycost)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and not c:IsCode(id) and c:CheckActivateEffect(false,true,false)~=nil
    card::is_normal_trap(d, c) && !card::is_code(d, c, &[ID]) && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// The `chkc` branch of `s.target` and `s.gytg`: `local te=e:GetLabelObject(); local tg=te:GetTarget(); return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)`
fn chkc_target(d: &mut Duel, a: &EffArgs, chkc: CardId) -> bool {
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    let tg = effect::get_target(d, te);
    match tg {
        Some(tg) => tg
            .call(
                d,
                &[
                    Value::Effect(a.e),
                    Value::Int(a.tp as i64),
                    a.eg.clone().map_or(Value::Nil, Value::Group),
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    Value::effect(a.re),
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                    Value::Int(0),
                    Value::Card(chkc),
                ],
            )
            .truthy(),
        None => false,
    }
}
/// The part of `s.target` and `s.gytg` after the selection: copies the chosen trap's activation.
fn copy_activation(d: &mut Duel, a: &EffArgs, g: CardSet) {
    // local te,ceg,cep,cev,cre,cr,crp=g:GetFirst():CheckActivateEffect(false,true,true)
    let tc = group::get_first(d, &g).nil();
    let (te, info) = card::check_activate_effect(d, tc, false, true, true).nil();
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // g:GetFirst():CreateEffectRelation(e)
    card::create_effect_relation(d, tc, a.e);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = tg {
        let (ceg, cep, cev, cre, cr, crp) = match info {
            Some(ev) => (
                ev.event_cards.clone().map_or(Value::Nil, Value::Group),
                Value::Int(ev.event_player as i64),
                Value::Int(ev.event_value as i64),
                Value::effect(ev.reason_effect),
                Value::Int(ev.reason as i64),
                Value::Int(ev.reason_player as i64),
            ),
            None => (Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil),
        };
        tg.call(
            d,
            &[Value::Effect(a.e), Value::Int(a.tp as i64), ceg, cep, cev, cre, cr, crp, Value::Int(1)],
        );
    }
    // te:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then ... end
    if let Some(chkc) = a.chkc() {
        return chkc_target(d, a, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    copy_activation(d, a, g);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e);
    // if not te or not te:GetHandler():IsRelateToEffect(e) then return end
    let Some(te) = te.as_effect() else { return };
    let th = effect::get_handler(d, te).nil();
    if !card::is_relate_to_effect(d, th, a.e) {
        return;
    }
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = op {
        op.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
            ],
        );
    }
}
/// `s.gycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gycost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then ... end
    if let Some(chkc) = a.chkc() {
        return chkc_target(d, a, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    copy_activation(d, a, g);
    true
}
