//! Twin-Sword Marauder
use super::prelude::*;

pub const ID: u32 = 40225398;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetOperation(s.caop)
    effect::set_operation(d, e1, Some(operation(caop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.caop(e,tp,eg,ep,ev,re,r,rp)`
fn caop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if Duel.GetAttacker()==c and bc and (bc:GetBattlePosition()&POS_DEFENSE)~=0
    //     and c:IsRelateToBattle() and c:CanChainAttack() then
    if duel::get_attacker(d) != Some(c) {
        return;
    }
    let Some(bc) = bc else {
        return;
    };
    if (card::get_battle_position(d, bc) & POS_DEFENSE) == 0 {
        return;
    }
    if card::is_relate_to_battle(d, c) && card::can_chain_attack(d, c, None, false) {
        // Duel.ChainAttack()
        duel::chain_attack(d, None);
    }
}
