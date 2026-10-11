//! Xyz Unit
use super::prelude::*;

pub const ID: u32 = 13032689;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsType,TYPE_XYZ))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_type(d, c, TYPE_XYZ))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // ATK increase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cost replacement
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.rcon)
    effect::set_condition(d, e2, Some(condition(rcon)));
    // e2:SetOperation(s.rop)
    effect::set_operation(d, e2, Some(condition(rop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetRank()*200
    card::get_rank(d, c) as i64 * 200
}
// function s.rcon(e,tp,eg,ep,ev,re,r,rp)
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_COST)~=0 and re:IsActivated() and re:IsActiveType(TYPE_XYZ)
    if (a.r & REASON_COST) == 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) || !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    // and ep==e:GetOwnerPlayer() and e:GetHandler():GetEquipTarget()==re:GetHandler()
    //     and re:GetHandler():GetOverlayCount()>=ev-1
    let h = effect::get_handler(d, a.e).nil();
    let rc = effect::get_handler(d, re).nil();
    a.ep == effect::get_owner_player(d, a.e)
        && card::get_equip_target(d, h) == Some(rc)
        && card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
// function s.rop(e,tp,eg,ep,ev,re,r,rp)
fn rop(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    // (a Lua number is true even when it is 0, so the operation returns true)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
