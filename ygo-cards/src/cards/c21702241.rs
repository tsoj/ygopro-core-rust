//! Vengeful Servant
//! 反目の従者
use super::prelude::*;

pub const ID: u32 = 21702241;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_CONTROL_CHANGED)
    effect::set_code(d, e3, EVENT_CONTROL_CHANGED);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tg = card::get_equip_target(d, h);
    // return tg and eg:IsContains(tg)
    match tg {
        Some(tg) => group::is_contains(&a.eg.clone().nil(), tg),
        None => false,
    }
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) and not e:GetHandler():IsStatus(STATUS_CHAINING) end
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, h, a.e) && !card::is_status(d, h, STATUS_CHAINING);
    }
    // local ec=e:GetHandler():GetEquipTarget()
    let ec = card::get_equip_target(d, h);
    // if not ec then return false end
    let Some(ec) = ec else {
        return false;
    };
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,ec:GetControler(),ec:GetBaseAttack())
    let p = card::get_controler(d, ec);
    let atk = card::get_base_attack(d, ec);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, p, atk);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c).nil();
    // Duel.Damage(ec:GetControler(),ec:GetBaseAttack(),REASON_EFFECT)
    let p = card::get_controler(d, ec);
    let atk = card::get_base_attack(d, ec) as i64;
    duel::damage(d, p, atk, REASON_EFFECT, false, None);
}
