//! Dark Lucius LV8
//! Dark Lucius LV8
use super::prelude::*;

pub const ID: u32 = 58206034;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12817939], lv: Some((8, SET_DARK_LUCIUS)), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // redirect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_BATTLE_DESTROY_REDIRECT)
    effect::set_code(d, e2, EFFECT_BATTLE_DESTROY_REDIRECT);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.discon(e)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(12817939)~=0
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, 12817939) != 0
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let mut target = duel::get_attack_target(d);
    // if d==c then d=Duel.GetAttacker() end
    if target == Some(c) {
        target = duel::get_attacker(d);
    }
    // if d and d:IsStatus(STATUS_BATTLE_DESTROYED) and d:IsType(TYPE_EFFECT) and not c:IsStatus(STATUS_BATTLE_DESTROYED) then
    if let Some(dc) = target {
        if card::is_status(d, dc, STATUS_BATTLE_DESTROYED)
            && card::is_type(d, dc, TYPE_EFFECT)
            && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            // d:RegisterEffect(e1)
            card::register_effect(d, dc, e1, false);
        }
    }
}
