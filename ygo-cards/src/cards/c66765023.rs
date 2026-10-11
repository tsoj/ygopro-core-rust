//! Flying Elephant
use super::prelude::*;

pub const ID: u32 = 66765023;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add win effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp
        //   and Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFlagEffect(id)==0
        return card::is_reason(d, c, REASON_EFFECT)
            && card::get_reason_player(d, c) != a.tp
            && duel::is_turn_player(d, 1 - a.tp)
            && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // return true
    true
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_SELF_TURN)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 1);
    // e1:SetCondition(s.wincon)
    effect::set_condition(d, e1, Some(condition(wincon)));
    // e1:SetOperation(s.winop)
    effect::set_operation(d, e1, Some(operation(winop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.wincon(e,tp,eg,ep,ev,re,r,rp)`
fn wincon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.GetAttackTarget()==nil
    a.ep != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.winop(e,tp,eg,ep,ev,re,r,rp)`
fn winop(d: &mut Duel, a: &EffArgs) {
    // Duel.Win(tp,WIN_REASON_FLYING_ELEPHANT)
    duel::win(d, a.tp, WIN_REASON_FLYING_ELEPHANT);
}
