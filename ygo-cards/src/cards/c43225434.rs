//! Battle Fusion
//! 決闘融合－バトル・フュージョン
use super::prelude::*;

pub const ID: u32 = 43225434;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a_card = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return at and ((a:IsControler(tp) and a:IsType(TYPE_FUSION))
    //     or (at:IsControler(tp) and at:IsFaceup() and at:IsType(TYPE_FUSION)))
    let Some(at) = at else { return false };
    let a_card = a_card.nil();
    (card::is_controler(d, a_card, a.tp) && card::is_type(d, a_card, TYPE_FUSION))
        || (card::is_controler(d, at, a.tp) && card::is_faceup(d, at) && card::is_type(d, at, TYPE_FUSION))
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let mut a_card = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let mut at = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a,at=at,a end
    if card::is_controler(d, a_card.nil(), 1 - a.tp) {
        std::mem::swap(&mut a_card, &mut at);
    }
    // if not a:IsRelateToBattle() or a:IsFacedown() or not at:IsRelateToBattle() or at:IsFacedown() then return end
    if !card::is_relate_to_battle(d, a_card.nil())
        || card::is_facedown(d, a_card.nil())
        || !card::is_relate_to_battle(d, at.nil())
        || card::is_facedown(d, at.nil())
    {
        return;
    }
    let a_card = a_card.nil();
    let at = at.nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // e1:SetValue(at:GetAttack())
    let atk = card::get_attack(d, at);
    effect::set_value(d, e1, atk as i64);
    // a:RegisterEffect(e1)
    card::register_effect(d, a_card, e1, false);
}
