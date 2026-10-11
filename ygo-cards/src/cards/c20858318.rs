//! Dark Scorpion Combination
use super::prelude::*;

pub const ID: u32 = 20858318;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[76922029, 6967870, 61587183, 48768179, 74153887], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,76922029),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,6967870),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,61587183),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,48768179),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,74153887),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[76922029])), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[6967870])), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[61587183])), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[48768179])), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[74153887])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).unwrap_or_default();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.affected)
    effect::set_target(d, e1, Some(effect_card_fn(affected)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetValue(400)
    effect::set_value(d, e2, 400);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.affected(e,c)`
fn affected(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.rdcon(e)`
fn rdcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetAttacker():GetFlagEffect(id)~=0 and Duel.GetAttackTarget()==nil
    let attacker = duel::get_attacker(d).nil();
    card::get_flag_effect(d, attacker, ID) != 0 && duel::get_attack_target(d).is_none()
}
