//! Gamushara
use super::prelude::*;

pub const ID: u32 = 85709845;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let tgt = duel::get_attack_target(d).nil();
    // if chk==0 then return d:IsDefensePos() and d:IsControler(tp) end
    if a.chk == Some(0) {
        return card::is_defense_pos(d, tgt) && card::is_controler(d, tgt, a.tp);
    }
    // Duel.SetTargetCard(d)
    duel::set_target_card(d, tgt);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsDefensePos() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_defense_pos(d, tc) {
        // Duel.ChangePosition(tc,POS_FACEUP_ATTACK)
        duel::change_position(d, tc, POS_FACEUP_ATTACK as u8, None, None, None, false);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_BATTLE_DESTROYED)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
        // e1:SetCondition(s.damcon)
        effect::set_condition(d, e1, Some(condition(damcon)));
        // e1:SetTarget(s.damtg)
        effect::set_target(d, e1, Some(cost_target(damtg)));
        // e1:SetOperation(s.damop)
        effect::set_operation(d, e1, Some(operation(damop)));
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    // return eg:IsContains(tc) and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
    let eg = a.eg.clone().nil();
    match duel::get_attacker(d) {
        Some(tc) => eg.contains(tc) && card::is_location(d, tc, LOCATION_GRAVE) && card::is_reason(d, tc, REASON_BATTLE),
        None => false,
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local atk=Duel.GetAttacker():GetBaseAttack()
    let attacker = duel::get_attacker(d).nil();
    let atk = card::get_base_attack(d, attacker);
    // if atk<0 then atk=0 end
    let atk = atk.max(0);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(atk)
    duel::set_target_param(d, atk as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
