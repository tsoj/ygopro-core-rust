//! Reflect Bounder
use super::prelude::*;

pub const ID: u32 = 2851070;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // return c==e:GetHandler() and c:GetBattlePosition()==POS_FACEUP_ATTACK
    match tc {
        Some(tc) if Some(tc) == effect::get_handler(d, a.e) => card::get_battle_position(d, tc) == POS_FACEUP_ATTACK,
        _ => false,
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk ==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local lp1=Duel.GetLP(p)
    let lp1 = duel::get_lp(d, p);
    // Duel.Damage(p,Duel.GetAttacker():GetAttack(),REASON_EFFECT)
    let attacker = duel::get_attacker(d).nil();
    let atk = card::get_attack(d, attacker);
    duel::damage(d, p, atk as i64, REASON_EFFECT, false, None);
    // local lp2=Duel.GetLP(p)
    let lp2 = duel::get_lp(d, p);
    // if lp2<lp1 then
    if lp2 < lp1 {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)~=0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) != 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
        duel::destroy(d, h, REASON_EFFECT, None, None);
    }
}
