//! Symph Amplifire
//! アンプリファイヤー
use super::prelude::*;

pub const ID: u32 = 75304793;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SYMPHONIC_WARRIOR], counter_place_list: &[0x35], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x35)
    card::enable_counter_permit(d, c, 0x35, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_SYMPHONIC_WARRIOR))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_SYMPHONIC_WARRIOR as u16]))));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // counter
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler():IsSetCard(SET_SYMPHONIC_WARRIOR) and not re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let Some(re) = a.re else {
        return false;
    };
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_SYMPHONIC_WARRIOR as u16]) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x35,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x35, 1, false);
}
/// `s.atkval(e,c)`: `return e:GetHandler():GetCounter(0x35)*100`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, 0x35) as i64 * 100
}
/// `s.filter(c)`: `return c:IsFaceup() and c:IsSetCard(SET_SYMPHONIC_WARRIOR)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SYMPHONIC_WARRIOR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let ct = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64;
    // local b1=Duel.IsCanRemoveCounter(tp,1,0,0x35,5,REASON_COST)
    let b1 = duel::is_can_remove_counter(d, tp, 1, 0, 0x35, 5, REASON_COST);
    // local b2=Duel.IsCanRemoveCounter(tp,1,0,0x35,7,REASON_COST)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
    let b2 = duel::is_can_remove_counter(d, tp, 1, 0, 0x35, 7, REASON_COST) && duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    // if chk==0 then return ct>0 and (b1 or b2) end
    if a.chk == Some(0) {
        return ct > 0 && (b1 || b2);
    }
    // local op=0 (overwritten in every branch below)
    // if b1 and b2 then
    let op: i64 = if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,0))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]) as i64
    // else
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) as i64 + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==0 then
    if op == 0 {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e, CATEGORY_DAMAGE);
        // Duel.RemoveCounter(tp,1,0,0x35,5,REASON_COST)
        duel::remove_counter(d, tp, 1, 0, 0x35, 5, REASON_COST);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - tp);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*300)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (ct * 300) as i32);
    } else {
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, e, CATEGORY_REMOVE);
        // Duel.RemoveCounter(tp,1,0,0x35,7,REASON_COST)
        duel::remove_counter(d, tp, 1, 0, 0x35, 7, REASON_COST);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,0,LOCATION_ONFIELD|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    }
    true
}
/// `s.rmfilter(c)`: `return c:IsAbleToRemove() and aux.SpElimFilter(c,false,true)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, true)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let ct = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64;
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
        let p = info[0].int() as u8;
        // Duel.Damage(p,ct*300,REASON_EFFECT)
        duel::damage(d, p, ct * 300, REASON_EFFECT, false, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,ct,nil)
        let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, ct as u16, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
