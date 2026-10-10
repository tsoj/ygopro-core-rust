//! Ultimate Axon Kicker
use super::prelude::*;

pub const ID: u32 = 40101111;
pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.ffilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_PSYCHIC))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, Some(fc), st, Some(tp)))),
        ],
    );
    // pierce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e2, EFFECT_PIERCE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be destroyed
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // fusion success
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e4, CATEGORY_RECOVER);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetCondition(s.recon)
    effect::set_condition(d, e4, Some(condition(recon)));
    // e4:SetTarget(s.rectg)
    effect::set_target(d, e4, Some(cost_target(rectg)));
    // e4:SetOperation(s.recop)
    effect::set_operation(d, e4, Some(operation(recop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // spsummon condition
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e5, EFFECT_SPSUMMON_CONDITION);
    // e5:SetValue(s.splimit)
    effect::set_value_fn(d, e5, lua_fn(splimit));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if e:GetHandler():IsLocation(LOCATION_EXTRA) then
    let h = effect::get_handler(d, a.e(0)).nil();
    if card::is_location(d, h, LOCATION_EXTRA) {
        // return (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
        return (a.int(3) & SUMMON_TYPE_FUSION as i64) == SUMMON_TYPE_FUSION as i64;
    }
    // return true
    true
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_SYNCHRO,fc,sumtype,tp) and c:IsRace(RACE_PSYCHIC,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(fc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_PSYCHIC, Some(fc), sumtype, Some(tp))
}
/// `s.recon(e,tp,eg,ep,ev,re,r,rp)`
fn recon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bc = bc.nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rec=e:GetHandler():GetBattleTarget():GetAttack()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    let mut rec = card::get_attack(d, bc) as i64;
    // if rec<0 then rec=0 end
    if rec < 0 {
        rec = 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, rec as i32);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
