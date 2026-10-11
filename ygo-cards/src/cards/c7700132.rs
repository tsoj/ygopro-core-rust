//! Worm Noble
use super::prelude::*;

pub const ID: u32 = 7700132;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FLIP+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_FLIP + EFFECT_TYPE_SINGLE);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsPhase(PHASE_DAMAGE) and e:GetHandler()==Duel.GetAttackTarget() then
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_phase(d, PHASE_DAMAGE) && Some(h) == duel::get_attack_target(d) {
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - a.tp);
        // Duel.SetTargetParam(Duel.GetAttacker():GetAttack()/2)
        let attacker = duel::get_attacker(d).nil();
        let atk = card::get_attack(d, attacker);
        duel::set_target_param(d, lua_int(atk as f64 / 2.0) as u32);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,Duel.GetAttacker():GetAttack()/2)
        let attacker = duel::get_attacker(d).nil();
        let atk = card::get_attack(d, attacker);
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, lua_int(atk as f64 / 2.0) as i32);
    }
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPhase(PHASE_DAMAGE) and e:GetHandler()==Duel.GetAttackTarget() then
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_phase(d, PHASE_DAMAGE) && Some(h) == duel::get_attack_target(d) {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    }
}
