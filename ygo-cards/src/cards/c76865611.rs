//! Morphtronic Lantron
use super::prelude::*;

pub const ID: u32 = 76865611;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond_attackpos)));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.refval)
    effect::set_value_fn(d, e1, lua_fn(refval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    // e2:SetOperation(s.check)
    effect::set_operation(d, e2, Some(operation(check)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetCondition(s.cond)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetOperation(s.opd)
    effect::set_operation(d, e3, Some(operation(opd)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.condition(e)`
fn cond_attackpos(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
/// `s.refval(e,re,val,r,rp,rc)`
fn refval(d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_EFFECT)~=0 and rp==1-e:GetHandlerPlayer()
    let r = a.int(3) as u32;
    let rp = a.p(4);
    let hp = effect::get_handler_player(d, a.e(0));
    (r & REASON_EFFECT) != 0 && rp == 1 - hp
}
/// `s.check(e,tp,eg,ep,ev,re,r,rp)`
fn check(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsDisabled() and c:IsDefensePos() then e:SetLabel(1)
    if !card::is_disabled(d, c) && card::is_defense_pos(d, c) {
        effect::set_label(d, a.e, &[1]);
    }
    // else e:SetLabel(0) end
    else {
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.cond(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetLabel()==1
    //     and e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, lo) != 1 {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.opd(e,tp,eg,ep,ev,re,r,rp)`
fn opd(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
