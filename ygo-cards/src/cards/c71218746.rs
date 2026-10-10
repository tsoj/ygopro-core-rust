//! Drillroid
use super::prelude::*;

pub const ID: u32 = 71218746;

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
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetTarget(s.targ)
    effect::set_target(d, e1, Some(cost_target(targ)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.targ(e,tp,eg,ep,ev,re,r,rp,chk)`
fn targ(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if chk ==0 then return Duel.GetAttacker()==e:GetHandler() and t~=nil and not t:IsAttackPos() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e);
        return duel::get_attacker(d) == h && t.is_some_and(|t| !card::is_attack_pos(d, t));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,t,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, t.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if t~=nil and t:IsRelateToBattle() and not t:IsAttackPos() then
    if let Some(t) = t {
        if card::is_relate_to_battle(d, t) && !card::is_attack_pos(d, t) {
            // Duel.Destroy(t,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(t), REASON_EFFECT, None, None);
        }
    }
}
