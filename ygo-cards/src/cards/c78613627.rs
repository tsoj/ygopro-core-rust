//! Des Kangaroo
//! デス・カンガルー
use super::prelude::*;

pub const ID: u32 = 78613627;

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
    let h = effect::get_handler(d, a.e).nil();
    // return Duel.GetAttackTarget()==e:GetHandler() and (e:GetHandler():GetBattlePosition()&POS_DEFENSE)~=0
    //     and Duel.GetAttacker():GetAttack()<e:GetHandler():GetDefense()
    duel::get_attack_target(d) == Some(h)
        && (card::get_battle_position(d, h) & POS_DEFENSE as u32) != 0
        && card::get_attack(d, duel::get_attacker(d).nil()) < card::get_defense(d, h)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    let atk = duel::get_attacker(d).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, atk, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // if not a:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, atk) {
        return;
    }
    // Duel.Destroy(a,REASON_EFFECT)
    duel::destroy(d, atk, REASON_EFFECT, None, None);
}
