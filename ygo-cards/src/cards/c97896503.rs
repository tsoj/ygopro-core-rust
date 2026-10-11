//! Zubaba Knight
use super::prelude::*;

pub const ID: u32 = 97896503;

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
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chk==0 then return Duel.GetAttacker()==e:GetHandler()
    //     and d~=nil and d:IsPosition(POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_attacker(d) == Some(h)
            && at.is_some_and(|at| card::is_position(d, at, POS_FACEUP_DEFENSE as u32));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, at.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if d~=nil and d:IsRelateToBattle() and d:IsDefensePos() then
    if let Some(at) = at {
        if card::is_relate_to_battle(d, at) && card::is_defense_pos(d, at) {
            // Duel.Destroy(d,REASON_EFFECT)
            duel::destroy(d, at, REASON_EFFECT, None, None);
        }
    }
}
