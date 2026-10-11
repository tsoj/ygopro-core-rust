//! Rising Sun Slash
use super::prelude::*;

pub const ID: u32 = 94807487;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], counter_place_list: &[0x31], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x31)
    card::enable_counter_permit(d, c, 0x31, None, None);
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_UTOPIA))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_UTOPIA as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Untargetable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // counter
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e4, CATEGORY_COUNTER);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_ATTACK_DISABLED)
    effect::set_code(d, e4, EVENT_ATTACK_DISABLED);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // atk
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e5, EFFECT_UPDATE_ATTACK);
    // e5:SetValue(s.value)
    effect::set_value_fn(d, e5, effect_card_value(value));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // remove overlay replace
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e6, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.rcon)
    effect::set_condition(d, e6, Some(condition(rcon)));
    // e6:SetOperation(s.rop)
    effect::set_operation(d, e6, Some(operation_count(rop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x31,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x31, 1, false);
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandler():GetCounter(0x31)*500
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, 0x31) as i64 * 500
}
/// `s.rcon(e,tp,eg,ep,ev,re,r,rp)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, _tp) = (a.e, a.tp);
    // return (r&REASON_COST)~=0 and re:GetHandler():IsType(TYPE_XYZ)
    //     and ep==e:GetOwnerPlayer() and e:GetHandler():GetEquipTarget()==re:GetHandler()
    //     and re:GetHandler():GetOverlayCount()>=ev-1
    if (a.r & REASON_COST) == 0 {
        return false;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if !card::is_type(d, rc, TYPE_XYZ) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    a.ep == effect::get_owner_player(d, e)
        && card::get_equip_target(d, h) == Some(rc)
        && card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
/// `s.rop(e,tp,eg,ep,ev,re,r,rp)`
fn rop(d: &mut Duel, a: &EffArgs) -> i64 {
    // return Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None) as i64
}
