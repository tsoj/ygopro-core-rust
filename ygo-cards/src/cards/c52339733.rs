//! Lindbloom
use super::prelude::*;

pub const ID: u32 = 52339733;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // return d and a:GetControler()~=d:GetControler()
    match df {
        None => false,
        Some(df) => {
            let a_ctrl = card::get_controler(d, at.nil());
            let d_ctrl = card::get_controler(d, df);
            a_ctrl != d_ctrl
        }
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // if a:IsFaceup() and a:IsRelateToBattle() and d:IsFaceup() and d:IsRelateToBattle() then
    let at = at.nil();
    if card::is_faceup(d, at) && card::is_relate_to_battle(d, at) && card::is_faceup(d, df.nil()) && card::is_relate_to_battle(d, df.nil()) {
        let df = df.nil();
        // if not a:IsLinkMonster() then
        if !card::is_link_monster(d, at) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(a:GetDefense())
            let def = card::get_defense(d, at);
            effect::set_value(d, e1, def as i64);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // a:RegisterEffect(e1)
            card::register_effect(d, at, e1, false);
        }
        // if not d:IsLinkMonster() then
        if !card::is_link_monster(d, df) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(d:GetDefense())
            let def = card::get_defense(d, df);
            effect::set_value(d, e1, def as i64);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // d:RegisterEffect(e1)
            card::register_effect(d, df, e1, false);
        }
    }
}
