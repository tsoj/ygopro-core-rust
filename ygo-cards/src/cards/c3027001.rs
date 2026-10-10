//! Fake Trap
use super::prelude::*;

pub const ID: u32 = 3027001;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`: `return c:IsTrap() and c:IsControler(tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_trap(d, c) && card::is_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp then return false end
    if a.rp == a.tp {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and #tg==tc and tg:IsExists(s.cfilter,1,e:GetHandler(),tp)
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) else { return false };
    let Some(tg) = info.op_cards.clone() else { return false };
    if tg.len() != info.op_count as usize {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &tg, &|d, x| cfilter(d, x, a.tp), 1, c.into())
}
/// `s.cffilter(c,tp)`: `return c:IsFacedown() and c:IsControler(tp)`
fn cffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_facedown(d, c) && card::is_controler(d, c, tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // local fg=tg:Filter(s.cffilter,nil,tp)
    let tg = duel::get_operation_info(d, a.ev, CATEGORY_DESTROY).and_then(|i| i.op_cards).nil();
    let fg = group::filter(d, &tg, &|d, c| cffilter(d, c, a.tp), NO_EX);
    // Duel.ConfirmCards(1-tp,fg)
    duel::confirm_cards(d, 1 - a.tp, fg);
    // local reg=tg:Filter(s.cfilter,e:GetHandler(),tp)
    let h = effect::get_handler(d, a.e).nil();
    let reg = group::filter(d, &tg, &|d, c| cfilter(d, c, a.tp), h.into());
    // local tc=reg:GetFirst()
    let _ = group::get_first(d, &reg);
    // for tc in aux.Next(reg) do
    for tc in group::iter(d, &reg) {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repvalue)
    effect::set_value_fn(d, e1, effect_card_fn(repvalue));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // e1:SetLabelObject(re)
    effect::set_label_object(d, e1, a.re.map_or(Value::Nil, Value::Effect));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repfilter(c)`: `return c:GetFlagEffect(id)~=0`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return re==e:GetLabelObject() and eg:IsExists(s.repfilter,1,nil) end
    if a.chk == Some(0) {
        let lo = effect::get_label_object(d, a.e).as_effect();
        if a.re == lo {
            let eg = a.eg.clone().nil();
            return group::is_exists(d, &eg, &|d, c| repfilter(d, c), 1, NO_EX);
        }
        return false;
    }
    // return true
    true
}
/// `s.repvalue(e,c)`: `return c:GetFlagEffect(id)~=0`
fn repvalue(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT|REASON_REPLACE)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, CardSet::from_card(c), REASON_EFFECT | REASON_REPLACE, None, None);
}
