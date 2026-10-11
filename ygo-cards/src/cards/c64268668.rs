//! Cyber Ogre
use super::prelude::*;

pub const ID: u32 = 64268668;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE, None);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return Duel.IsBattlePhase()
    //     and ((a and a:IsControler(tp) and a:IsFaceup() and a:IsCode(id))
    //     or (d and d:IsControler(tp) and d:IsFaceup() and d:IsCode(id)))
    duel::is_battle_phase(d, None) && (is_own_ogre(d, at, a.tp) || is_own_ogre(d, dt, a.tp))
}
/// `a and a:IsControler(tp) and a:IsFaceup() and a:IsCode(id)`
fn is_own_ogre(d: &mut Duel, c: Option<CardId>, tp: u8) -> bool {
    match c {
        Some(c) => card::is_controler(d, c, tp) && card::is_faceup(d, c) && card::is_code(d, c, &[ID]),
        None => false,
    }
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, h, REASON_COST | REASON_DISCARD, None, None);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if chk==0 then
    if a.chk == Some(0) {
        // if a:IsControler(tp) then return a:IsCanBeEffectTarget(e)
        if card::is_controler(d, at.nil(), a.tp) {
            return card::is_can_be_effect_target(d, at.nil(), Some(a.e));
        }
        // else return d:IsCanBeEffectTarget(e) end
        return card::is_can_be_effect_target(d, dt.nil(), Some(a.e));
    }
    // if a:IsControler(tp) then return Duel.SetTargetCard(a)
    if card::is_controler(d, at.nil(), a.tp) {
        duel::set_target_card(d, at.nil());
    } else {
        // else return Duel.SetTargetCard(d) end
        duel::set_target_card(d, dt.nil());
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.NegateAttack()
        duel::negate_attack(d);
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(2000)
        effect::set_value(d, e1, 2000);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // e2:SetOperation(s.resetop)
        effect::set_operation(d, e2, Some(operation(resetop)));
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
// function s.resetop(e,tp,eg,ep,ev,re,r,rp)
fn resetop(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():Reset()
    let l = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, l);
    // e:Reset()
    effect::reset(d, a.e);
}
