//! Ninja Grandmaster Sasuke
use super::prelude::*;

pub const ID: u32 = 4041838;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_START);
    effect::set_target(d, e1, Some(cost_target(targ)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.targ(e,tp,eg,ep,ev,re,r,rp,chk)`
fn targ(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    if a.chk == Some(0) {
        // return Duel.GetAttacker()==e:GetHandler() and d~=nil and d:IsFaceup() and d:IsDefensePos() and d:IsRelateToBattle()
        let attacker = duel::get_attacker(d);
        let h = effect::get_handler(d, a.e);
        if attacker != h {
            return false;
        }
        let Some(dt) = dt else { return false };
        return card::is_faceup(d, dt) && card::is_defense_pos(d, dt) && card::is_relate_to_battle(d, dt);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dt.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if d~=nil and d:IsRelateToBattle() and d:IsDefensePos() then
    if let Some(dt) = dt {
        if card::is_relate_to_battle(d, dt) && card::is_defense_pos(d, dt) {
            // Duel.Destroy(d,REASON_EFFECT)
            duel::destroy(d, dt, REASON_EFFECT, None, None);
        }
    }
}
