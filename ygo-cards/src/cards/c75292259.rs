//! Great Poseidon Beetle
use super::prelude::*;

pub const ID: u32 = 75292259;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // chain attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c==Duel.GetAttacker() and bc and bc:IsRelateToBattle() and bc:GetBattlePosition()==POS_FACEUP_ATTACK
    //     and c:CanChainAttack(3)
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    let Some(bc) = bc else {
        return false;
    };
    card::is_relate_to_battle(d, bc)
        && card::get_battle_position(d, bc) == POS_FACEUP_ATTACK
        && card::can_chain_attack(d, c, Some(3), false)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // Duel.ChainAttack(e:GetHandler():GetBattleTarget())
    let c = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, c);
    duel::chain_attack(d, bt);
}
