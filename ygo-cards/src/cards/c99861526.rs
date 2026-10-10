//! Submarineroid
use super::prelude::*;

pub const ID: u32 = 99861526;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage reduce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetValue(s.rval)
    effect::set_value_fn(d, e2, lua_fn(rval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetCondition(s.poscon)
    effect::set_condition(d, e3, Some(condition(poscon)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rdcon(e)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetAttackTarget()==nil and c:GetEffectCount(EFFECT_DIRECT_ATTACK)<2
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_attack_target(d).is_none() && card::get_effect_count(d, c, EFFECT_DIRECT_ATTACK) < 2 && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.rval(e,damp)`
fn rval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let damp = a.int(1);
    // if damp==1-e:GetHandlerPlayer() then
    let tp = effect::get_handler_player(d, e) as i64;
    if damp == 1 - tp {
        // return e:GetHandler():GetBaseAttack()
        let h = effect::get_handler(d, e).nil();
        card::get_base_attack(d, h) as i64
    } else {
        // return -1
        -1
    }
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRelateToBattle() and c:IsAttackPos()
    card::is_relate_to_battle(d, c) && card::is_attack_pos(d, c)
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.ChangePosition(e:GetHandler(),POS_FACEUP_DEFENSE)
        duel::change_position(d, h, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
