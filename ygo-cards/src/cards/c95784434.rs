//! Molting Escape
use super::prelude::*;

pub const ID: u32 = 95784434;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsRace,RACE_REPTILE))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_REPTILE))), None, None, None, None, None, None);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_COUNT);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetValue(s.valcon)
    effect::set_value_fn(d, e3, lua_fn(valcon));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atkup
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    // e4:SetCondition(s.adcon)
    effect::set_condition(d, e4, Some(condition(adcon)));
    // e4:SetOperation(s.adop)
    effect::set_operation(d, e4, Some(operation(adop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.valcon(e,re,r,rp)`
fn valcon(d: &mut Duel, a: Args) -> bool {
    // if (r&REASON_BATTLE)~=0 then
    if (a.int(2) as u32 & REASON_BATTLE) != 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
        let h = effect::get_handler(d, a.e(0)).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:GetEquipTarget():RegisterEffect(e1)
    let equip_target = card::get_equip_target(d, c).nil();
    card::register_effect(d, equip_target, e1, false);
}
