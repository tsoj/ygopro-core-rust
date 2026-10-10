//! Tsukumo Slash
use super::prelude::*;

pub const ID: u32 = 25334372;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let tgt = duel::get_attack_target(d);
    // if not d then return false end
    let Some(tgt) = tgt else {
        return false;
    };
    // return a:IsControler(tp) and a:GetAttack()<d:GetAttack() and Duel.GetLP(tp)~=Duel.GetLP(1-tp)
    card::is_controler(d, att, a.tp)
        && card::get_attack(d, att) < card::get_attack(d, tgt)
        && duel::get_lp(d, a.tp) != duel::get_lp(d, 1 - a.tp)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=Duel.GetAttacker()
    let c = duel::get_attacker(d).nil();
    // if c:IsFaceup() and c:IsRelateToBattle() then
    if card::is_faceup(d, c) && card::is_relate_to_battle(d, c) {
        // local atk=math.abs(Duel.GetLP(tp)-Duel.GetLP(1-tp))
        let atk = (duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp)).abs() as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
