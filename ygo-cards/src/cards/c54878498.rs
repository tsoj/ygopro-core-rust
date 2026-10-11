//! Kelbek
use super::prelude::*;

pub const ID: u32 = 54878498;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()==e:GetHandler() and not Duel.GetAttacker():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d) == Some(h) && {
        let att = duel::get_attacker(d).nil();
        !card::is_status(d, att, STATUS_BATTLE_DESTROYED)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,Duel.GetAttacker(),1,0,0)
    let att = duel::get_attacker(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, att, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // if not a:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, att) {
        return;
    }
    // Duel.SendtoHand(a,nil,REASON_EFFECT)
    duel::send_to_hand(d, CardSet::from_card(att), None, REASON_EFFECT, None);
}
