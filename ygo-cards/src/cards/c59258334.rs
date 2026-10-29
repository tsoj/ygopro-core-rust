//! Brainwashing Beam
use super::prelude::*;

pub const ID: u32 = 59258334;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_list: &[COUNTER_A],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(aux.PersistentTgCon)
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e3, EFFECT_SET_CONTROL);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e3, Some(aux::persistent_target_filter()));
    // e3:SetValue(s.tg)
    effect::set_value_fn(d, e3, effect_value(tg));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy2
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetCondition(s.descon2)
    effect::set_condition(d, e5, Some(condition(descon2)));
    // e5:SetOperation(s.desop2)
    effect::set_operation(d, e5, Some(operation(desop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // remove counter
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.rccon)
    effect::set_condition(d, e6, Some(condition(rccon)));
    // e6:SetOperation(s.rcop)
    effect::set_operation(d, e6, Some(operation(rcop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCounter(COUNTER_A)>0 and c:IsControlerCanBeChanged()
    card::get_counter(d, c, COUNTER_A as u16) > 0 && card::is_controler_can_be_changed(d, c, false, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS):GetFirst()
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let tcs = info[0].as_group().cloned().nil();
    let tc = group::get_first(d, &tcs);
    // if c:IsRelateToEffect(re) and tc and tc:IsFaceup() and tc:GetCounter(COUNTER_A)>0 and tc:IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, c, a.re.nil()) {
        if let Some(tc) = tc {
            if card::is_faceup(d, tc)
                && card::get_counter(d, tc, COUNTER_A as u16) > 0
                && card::is_relate_to_effect(d, tc, a.re.nil())
            {
                // c:SetCardTarget(tc)
                card::set_card_target(d, c, tc);
            }
        }
    }
}
// function s.tg(e,c)
fn tg(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandlerPlayer()
    effect::get_handler_player(d, e) as i64
}
// function s.descon(e)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCardTargetCount()==0 then return false end
    if card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // return c:GetFirstCardTarget():GetCounter(COUNTER_A)==0
    let tc = card::get_first_card_target(d, c).nil();
    card::get_counter(d, tc, COUNTER_A as u16) == 0
}
// function s.descon2(e,tp,eg,ep,ev,re,r,rp)
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    match card::get_first_card_target(d, h) {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
        }
        None => false,
    }
}
// function s.desop2(e,tp,eg,ep,ev,re,r,rp)
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
// function s.rccon(e,tp,eg,ep,ev,re,r,rp)
fn rccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::get_first_card_target(d, h).is_some()
}
// function s.rcop(e,tp,eg,ep,ev,re,r,rp)
fn rcop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, h).nil();
    // tc:RemoveCounter(tp,COUNTER_A,1,REASON_EFFECT)
    card::remove_counter(d, tc, tp, COUNTER_A as u16, 1, REASON_EFFECT);
    // Duel.RaiseEvent(e:GetHandler(),EVENT_REMOVE_COUNTER+COUNTER_A,e,REASON_EFFECT,tp,tp,1)
    duel::raise_event(d, h, EVENT_REMOVE_COUNTER + COUNTER_A, Some(a.e), REASON_EFFECT, tp, tp, 1);
}
