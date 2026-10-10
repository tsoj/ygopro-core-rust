//! Sasuke Samurai
use super::prelude::*;

pub const ID: u32 = 16222645;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return e:GetHandler()==Duel.GetAttacker() and d and d:IsFacedown() and d:IsDefensePos()
    if effect::get_handler(d, a.e) != duel::get_attacker(d) {
        return false;
    }
    let Some(tc) = at else { return false };
    card::is_facedown(d, tc) && card::is_defense_pos(d, tc)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttackTarget(),1,0,0)
    let at = duel::get_attack_target(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() then
    if card::is_relate_to_battle(d, tc) {
        // Duel.Destroy(d,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
