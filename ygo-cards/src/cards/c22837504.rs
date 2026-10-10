//! Mist Valley Soldier
use super::prelude::*;

pub const ID: u32 = 22837504;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // return hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetTarget(s.rettg)
    effect::set_target(d, e1, Some(cost_target(rettg)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not e:GetHandler():IsRelateToBattle() then return false end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_battle(d, h) {
        return false;
    }
    // local t=nil
    // if ev==0 then t=Duel.GetAttackTarget()
    let t = if a.ev == 0 {
        duel::get_attack_target(d)
    } else {
        // else t=Duel.GetAttacker() end
        duel::get_attacker(d)
    };
    // e:SetLabelObject(t)
    effect::set_label_object(d, a.e, Value::card(t));
    // return t and t:IsRelateToBattle()
    match t {
        Some(t) => card::is_relate_to_battle(d, t),
        None => false,
    }
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetLabelObject(),1,0,0)
    let t = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, t.map(CardSet::from), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():IsRelateToBattle() then
    let t = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_battle(d, t) {
        // Duel.SendtoHand(e:GetLabelObject(),nil,REASON_EFFECT)
        duel::send_to_hand(d, t, None, REASON_EFFECT, None);
    }
}
