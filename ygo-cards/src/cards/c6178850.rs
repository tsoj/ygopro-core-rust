//! Fighting Spirit
use super::prelude::*;

pub const ID: u32 = 6178850;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // desrep
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.value(e,c)`
fn value(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)*300
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) as i64 * 300
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=c:GetEquipTarget()
    let tg = card::get_equip_target(d, c);
    // if chk==0 then return c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    //     and tg and tg:IsReason(REASON_BATTLE) end
    if a.chk == Some(0) {
        return card::is_destructable(d, c, Some(a.e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && tg.is_some_and(|tg| card::is_reason(d, tg, REASON_BATTLE));
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT|REASON_REPLACE)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
