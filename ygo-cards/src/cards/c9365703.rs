//! Speed Warrior
use super::prelude::*;

pub const ID: u32 = 9365703;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.dacon)
    effect::set_condition(d, e2, Some(condition(dacon)));
    // e2:SetOperation(s.daop)
    effect::set_operation(d, e2, Some(operation(daop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),0,1)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
}
/// `s.dacon(e,tp,eg,ep,ev,re,r,rp)`
fn dacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_BATTLE_STEP) and Duel.GetCurrentChain()==0 and e:GetHandler():GetFlagEffect(id)~=0
    let c = effect::get_handler(d, a.e).nil();
    duel::is_phase(d, PHASE_BATTLE_STEP) && duel::get_current_chain(d, false) == 0 && card::get_flag_effect(d, c, ID) != 0
}
/// `s.daop(e,tp,eg,ep,ev,re,r,rp)`
fn daop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(c:GetBaseAttack()*2)
    let atk = card::get_base_attack(d, c);
    effect::set_value(d, e1, atk as i64 * 2);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_BATTLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
