//! Three Thousand Needles
use super::prelude::*;

pub const ID: u32 = 7852878;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.GetAttackTarget()==c and ((c:IsOnField() and c:IsPosition(POS_DEFENSE)) or c:IsPreviousPosition(POS_DEFENSE))
    //     and Duel.GetAttacker():GetAttack()<c:GetDefense()
    if duel::get_attack_target(d) != Some(c) {
        return false;
    }
    if !((card::is_on_field(d, c) && card::is_position(d, c, POS_DEFENSE)) || card::is_previous_position(d, c, POS_DEFENSE)) {
        return false;
    }
    let attacker = duel::get_attacker(d).nil();
    let atk = card::get_attack(d, attacker);
    atk < card::get_defense(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, duel::get_attacker(d).map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // if not a:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, att) {
        return;
    }
    // Duel.Destroy(a,REASON_EFFECT)
    duel::destroy(d, att, REASON_EFFECT, None, None);
}
