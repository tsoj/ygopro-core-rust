//! Disc Fighter
//! 円盤闘士
use super::prelude::*;

pub const ID: u32 = 19612721;

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
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if chk==0 then return Duel.GetAttacker()==e:GetHandler() and t~=nil and t:IsDefensePos() and t:IsDefenseAbove(2000) end
    if a.chk == Some(0) {
        let attacker = duel::get_attacker(d);
        let handler = effect::get_handler(d, a.e);
        return attacker == handler
            && match t {
                Some(t) => card::is_defense_pos(d, t) && card::is_defense_above(d, t, 2000),
                None => false,
            };
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,t,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, t.map(CardSet::from), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if t~=nil and t:IsRelateToBattle() and not t:IsAttackPos() then
    if let Some(t) = t {
        if card::is_relate_to_battle(d, t) && !card::is_attack_pos(d, t) {
            // Duel.Destroy(t,REASON_EFFECT)
            duel::destroy(d, t, REASON_EFFECT, None, None);
        }
    }
}
