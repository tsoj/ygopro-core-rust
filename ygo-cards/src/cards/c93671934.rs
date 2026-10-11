//! Morale Boost
use super::prelude::*;

pub const ID: u32 = 93671934;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // recover
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_EQUIP)
    effect::set_code(d, e2, EVENT_EQUIP);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.reccon)
    effect::set_condition(d, e2, Some(condition(reccon)));
    // e2:SetTarget(s.rectg)
    effect::set_target(d, e2, Some(cost_target(rectg)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.reccon(e,tp,eg,ep,ev,re,r,rp)
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(eg:GetFirst():GetControler())
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    let ctl = card::get_controler(d, tc);
    effect::set_label(d, a.e, &[ctl as i64]);
    // return true
    true
}
// function s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,e:GetLabel(),1000)
    let p = effect::get_label(d, a.e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, p, 1000);
    true
}
// function s.recop(e,tp,eg,ep,ev,re,r,rp)
fn recop(d: &mut Duel, a: &EffArgs) {
    // Duel.Recover(e:GetLabel(),1000,REASON_EFFECT)
    let p = effect::get_label(d, a.e) as u8;
    duel::recover(d, p, 1000, REASON_EFFECT, false, None);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipTarget()~=nil or c:IsReason(REASON_LOST_TARGET)
    card::get_equip_target(d, c).is_some() || card::is_reason(d, c, REASON_LOST_TARGET)
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &filter, NO_EX);
    // if #g==0 then return false end
    if g.is_empty() {
        return false;
    }
    // local flag=0
    let mut flag = 0;
    // if g:IsExists(Card.IsControler,1,nil,0) then flag=flag+1 end
    if group::is_exists(d, &g, &|d, c| card::is_controler(d, c, 0), 1, NO_EX) {
        flag += 1;
    }
    // if g:IsExists(Card.IsControler,1,nil,1) then flag=flag+2 end
    if group::is_exists(d, &g, &|d, c| card::is_controler(d, c, 1), 1, NO_EX) {
        flag += 2;
    }
    // e:SetLabel(({0,1,PLAYER_ALL})[flag])
    let label = match flag {
        1 => 0,
        2 => 1,
        3 => PLAYER_ALL as i64,
        _ => 0,
    };
    effect::set_label(d, a.e, &[label]);
    // return true
    true
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,e:GetLabel(),1000)
    let p = effect::get_label(d, a.e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, p, 1000);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    // if e:GetLabel()~=1-tp then
    if label != (1 - a.tp) as i64 {
        // Duel.Damage(tp,1000,REASON_EFFECT,true)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, true, None);
    }
    // if e:GetLabel()~=tp then
    if label != a.tp as i64 {
        // Duel.Damage(1-tp,1000,REASON_EFFECT,true)
        duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, true, None);
    }
    // Duel.RDComplete()
    duel::rd_complete(d);
}
