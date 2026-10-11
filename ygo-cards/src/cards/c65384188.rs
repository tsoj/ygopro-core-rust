//! Stand-Off
use super::prelude::*;

pub const ID: u32 = 65384188;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsType(TYPE_EFFECT)
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_type(d, c, TYPE_EFFECT)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUPATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUPATTACK as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUPATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUPATTACK as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc1,tc2=Duel.GetFirstTarget()
    let targets = duel::get_first_target(d);
    let tc1 = targets.first().copied().nil();
    let tc2 = targets.get(1).copied().nil();
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and tc2:IsRelateToEffect(e) and tc2:IsFaceup() then
    if card::is_relate_to_effect(d, tc1, e) && card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc2, e) && card::is_faceup(d, tc2) {
        // local a=0
        let mut a_cnt = 0;
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // if not tc1:IsDisabled() then
        if !card::is_disabled(d, tc1) {
            // tc1:RegisterEffect(e1)
            card::register_effect(d, tc1, e1, false);
            // tc1:RegisterEffect(e2)
            card::register_effect(d, tc1, e2, false);
            // a=a+1
            a_cnt += 1;
        }
        // if not tc2:IsDisabled() then
        if !card::is_disabled(d, tc2) {
            // local e3=e1:Clone()
            let e3 = effect::clone(d, e1);
            // local e4=e2:Clone()
            let e4 = effect::clone(d, e2);
            // tc2:RegisterEffect(e3)
            card::register_effect(d, tc2, e3, false);
            // tc2:RegisterEffect(e4)
            card::register_effect(d, tc2, e4, false);
            // a=a+1
            a_cnt += 1;
        }
        // if tc1:IsDefensePos() or tc2:IsDefensePos() or a~=2 then return end
        if card::is_defense_pos(d, tc1) || card::is_defense_pos(d, tc2) || a_cnt != 2 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // s.reg(c,tc1,tc2)
        reg(d, c, tc1, tc2);
        // s.reg(c,tc2,tc1)
        reg(d, c, tc2, tc1);
    }
}
// s.reg(c,tc1,tc2)
fn reg(d: &mut Duel, c: CardId, tc1: CardId, tc2: CardId) {
    // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    let _ = card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc1:RegisterEffect(e1)
    card::register_effect(d, tc1, e1, false);
    // --Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(3000)
    effect::set_description(d, e2, 3000);
    // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| effcon(d, a.e(0)))));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetLabelObject(tc2)
    effect::set_label_object(d, e2, Value::Card(tc2));
    // tc1:RegisterEffect(e2)
    card::register_effect(d, tc1, e2, false);
    // --Cannot change their battle positions
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(3313)
    effect::set_description(d, e3, 3313);
    // e3:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
    effect::set_code(d, e3, EFFECT_CANNOT_CHANGE_POSITION);
    // tc1:RegisterEffect(e3)
    card::register_effect(d, tc1, e3, false);
    // --Cannot attack
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetDescription(3206)
    effect::set_description(d, e4, 3206);
    // e4:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    // tc1:RegisterEffect(e4)
    card::register_effect(d, tc1, e4, false);
    // --Unaffected by other card effects
    // local e5=e2:Clone()
    let e5 = effect::clone(d, e2);
    // e5:SetDescription(3100)
    effect::set_description(d, e5, 3100);
    // e5:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
    // e5:SetValue(s.efilter)
    effect::set_value_fn(d, e5, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    // tc1:RegisterEffect(e5)
    card::register_effect(d, tc1, e5, false);
}
// s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetFlagEffect(id)~=0 and not c:IsPosition(POS_FACEUP_ATTACK) then
    if card::get_flag_effect(d, c, ID) != 0 && !card::is_position(d, c, POS_FACEUP_ATTACK) {
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
    }
}
// s.effcon(e)
fn effcon(d: &mut Duel, e: EffectId) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0 and e:GetLabelObject():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, e).nil();
    let lo = effect::get_label_object(d, e).as_card().nil();
    card::get_flag_effect(d, h, ID) != 0 && card::get_flag_effect(d, lo, ID) != 0
}
// s.efilter(e,te)
fn efilter(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
