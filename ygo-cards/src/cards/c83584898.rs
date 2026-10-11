//! Darkworld Shackles
use super::prelude::*;

pub const ID: u32 = 83584898;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atk,def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    // e2:SetValue(100)
    effect::set_value(d, e2, 100);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e3, EFFECT_SET_DEFENSE);
    // e3:SetValue(100)
    effect::set_value(d, e3, 100);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // damage
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e6, CATEGORY_DAMAGE);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_STANDBY);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.damcon)
    effect::set_condition(d, e6, Some(condition(damcon)));
    // e6:SetTarget(s.damtg)
    effect::set_target(d, e6, Some(cost_target(damtg)));
    // e6:SetOperation(s.damop)
    effect::set_operation(d, e6, Some(operation(damop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// The controller of the equipped monster: `e:GetHandler():GetEquipTarget():GetControler()`.
fn equip_controller(d: &mut Duel, e: EffectId) -> u8 {
    let h = effect::get_handler(d, e).nil();
    let eqt = card::get_equip_target(d, h).nil();
    card::get_controler(d, eqt)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local p=e:GetHandler():GetEquipTarget():GetControler()
    let p = equip_controller(d, a.e);
    // Duel.SetTargetPlayer(p)
    duel::set_target_player(d, p);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,p,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, p, 500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p=e:GetHandler():GetEquipTarget():GetControler()
    let p = equip_controller(d, a.e);
    // local d=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, info[0].int(), REASON_EFFECT, false, None);
}
