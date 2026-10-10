//! Barbaroid, the Ultimate Battle Machine
use super::prelude::*;

pub const ID: u32 = 54702678;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,5)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, Some(fc), st, tp))), 5)]);
    // extra attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.ffilter(c,fc,sumtype,tp)
fn ffilter(d: &mut Duel, c: CardId, fc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROID,fc,sumtype,tp) and c:IsRace(RACE_MACHINE,fc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_ROID as u16], fc, sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_MACHINE, fc, sumtype, Some(tp))
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget() and e:GetHandler()==Duel.GetAttacker()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d).is_some() && Some(h) == duel::get_attacker(d)
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e1, RESET_EVENT | ((RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|(RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e2, RESET_EVENT | ((RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // bc:RegisterEffect(e2)
    card::register_effect(d, bc, e2, false);
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRelateToBattle() and c:GetBattleTarget():IsMonster()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::is_monster(d, bt)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
