//! Dimension Sphinx
use super::prelude::*;

pub const ID: u32 = 17787975;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,aux.FilterBoolFunction(Card.IsPosition,POS_FACEUP_ATTACK))
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_position(d, c, POS_FACEUP_ATTACK))),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
    );
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetHintTiming(TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e3, TIMING_BATTLE_PHASE, None);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_BATTLE_STEP)
    duel::is_phase(d, PHASE_BATTLE_STEP)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetFirstCardTarget()
    let tc = card::get_first_card_target(d, c);
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if chk==0 then return tc and Duel.GetAttackTarget()==tc
    //     and at and at:IsControler(1-tp)and at:GetAttack()>tc:GetAttack()
    //     and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let Some(tc) = tc else { return false };
        if duel::get_attack_target(d) != Some(tc) {
            return false;
        }
        let Some(at) = at else { return false };
        if !card::is_controler(d, at, 1 - tp) {
            return false;
        }
        if card::get_attack(d, at) <= card::get_attack(d, tc) {
            return false;
        }
        return card::get_flag_effect(d, c, ID) == 0;
    }
    // local dam=math.abs(at:GetAttack()-tc:GetAttack())
    let at = at.nil();
    let tc = tc.nil();
    let dam = (card::get_attack(d, at) - card::get_attack(d, tc)).abs();
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=c:GetFirstCardTarget()
    // if not tc then return false end
    let Some(tc) = card::get_first_card_target(d, c) else { return };
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:IsRelateToBattle() then
    if card::is_relate_to_battle(d, at) {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // Duel.Damage(p,math.abs(at:GetAttack()-tc:GetAttack()),REASON_EFFECT)
        let dam = (card::get_attack(d, at) - card::get_attack(d, tc)).abs();
        duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(tc) = card::get_first_card_target(d, h) else { return false };
    // return tc and eg:IsContains(tc)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
