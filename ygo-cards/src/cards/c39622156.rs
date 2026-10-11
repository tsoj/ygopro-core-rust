//! No.26 Spaceway Octobypass
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 39622156;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(26), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // battle restriction
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(Cost.DetachFromSelf(1))
    effect::set_target(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetOperation(s.btop)
    effect::set_operation(d, e1, Some(operation(btop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.ctcon)
    effect::set_condition(d, e3, Some(condition(ctcon)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.btop(e,tp,eg,ep,ev,re,r,rp)`
fn btop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| atktg(d, e, c))));
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
    // Duel.RegisterEffect(e1,p)
    duel::register_effect(d, e1, p);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetOperation(s.checkop)
    effect::set_operation(d, e2, Some(operation(checkop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_BATTLE, 1);
    // Duel.RegisterEffect(e2,p)
    duel::register_effect(d, e2, p);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsMonster))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_monster(d, c))));
    // e3:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_BATTLE, 1);
    // Duel.RegisterEffect(e3,p)
    duel::register_effect(d, e3, p);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e4, LOCATION_MZONE, 0);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // e4:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_BATTLE, 1);
    // Duel.RegisterEffect(e4,p)
    duel::register_effect(d, e4, p);
}
/// `s.atkcon(e)`: `return Duel.GetFlagEffect(e:GetHandlerPlayer(),id)~=0`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    duel::get_flag_effect(d, hp, ID) != 0
}
/// `s.atktg(e,c)`: `return c:GetFieldID()~=e:GetLabel()`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::get_field_id(d, c) as i64 != effect::get_label(d, e)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetFlagEffect(id)~=0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return;
    }
    // local fid=eg:GetFirst():GetFieldID()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let fid = card::get_field_id(d, first);
    // Duel.RegisterFlagEffect(e:GetHandlerPlayer(),id,RESETS_STANDARD_PHASE_END,0,1)
    let hp = effect::get_handler_player(d, a.e);
    duel::register_flag_effect(d, hp, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0);
    // e:GetLabelObject():SetLabel(fid)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[fid as i64]);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetAttackTarget()~=nil then return end
    if duel::get_attack_target(d).is_some() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    // eg:GetFirst():RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::register_flag_effect(d, first, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`: `return e:GetHandler():GetFlagEffect(id)>0`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.filter(c)`: `return c:GetFlagEffect(id+1)>0`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID + 1) > 0
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // if tc then Duel.GetControl(tc,1-p) end
    if let Some(tc) = tc {
        duel::get_control(d, tc, 1 - p, 0, 0, None, None);
    }
}
