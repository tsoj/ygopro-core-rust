//! Yubel - The Loving Defender Forever
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 47172959;

// s.listed_series={SET_YUBEL}
// s.material_location=LOCATION_ONFIELD
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YUBEL],
    material_location: Some(LOCATION_ONFIELD),
    ..ScriptInfo::DEFAULT
};

// chain data fields of this script (`e:GetChainData()`)
const CD_ATK_ON_FIELD: u8 = chain::field::CARD;
const CD_BATTLED_MONSTER: u8 = chain::field::CARD + 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon procedure
    // Fusion.AddProcMixRep(c,true,true,s.ffilter,1,99,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_YUBEL))
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))),
        1,
        99,
        vec![fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
            card::is_set_card_ctx(d, c, &[SET_YUBEL as u16], Some(fc), st, Some(tp))
        }))],
    );
    // Inflict 500 damage to your opponent for each Fusion Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(fusion_summoned_cond)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle or card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // You take no battle damage from battles involving this card
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e4, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Inflict damage to your opponent equal to their monster's ATK and banish that monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DAMAGE+CATEGORY_REMOVE)
    effect::set_category(d, e5, CATEGORY_DAMAGE + CATEGORY_REMOVE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e5, EVENT_DAMAGE_STEP_END);
    // e5:SetCondition(s.damrmcon)
    effect::set_condition(d, e5, Some(condition(damrmcon)));
    // e5:SetTarget(s.damrmtg)
    effect::set_target(d, e5, Some(cost_target(damrmtg)));
    // e5:SetOperation(s.damrmop)
    effect::set_operation(d, e5, Some(operation(damrmop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`: `c:IsType(TYPE_EFFECT,fc,sumtype,tp) and c:IsOnField()`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), sumtype, Some(tp)) && card::is_on_field(d, c)
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local mg=e:GetHandler():GetMaterial()
    let c = effect::get_handler(d, a.e).nil();
    let mg = card::get_material(d, c);
    // if chk==0 then return mg and #mg>0 end
    if a.chk == Some(0) {
        return !mg.is_empty();
    }
    // local dmg=#mg*500
    let dmg = mg.len() as i64 * 500;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dmg)
    duel::set_target_param(d, dmg as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dmg)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dmg as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p=Chain.GetTargetPlayer()
    let p = chain::get_target_player(d, 0).int() as u8;
    // local d=Chain.GetTargetParam()
    let dam = chain::get_target_param(d, 0).int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
/// `s.damrmcon(e,tp,eg,ep,ev,re,r,rp)`
fn damrmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    // if not bc then return false end
    let Some(bc) = card::get_battle_target(d, c) else {
        return false;
    };
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // return bc:IsControler(1-tp)
        card::is_controler(d, bc, 1 - a.tp)
    } else {
        // return bc:IsPreviousControler(1-tp)
        card::is_previous_controler(d, bc, 1 - a.tp)
    }
}
/// `s.damrmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damrmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local cd=e:GetChainData()
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // local dam=0
    // if bc:IsRelateToBattle() then
    let bc = bc.nil();
    let dam: i64 = if card::is_relate_to_battle(d, bc) {
        // dam=bc:GetAttack()
        let dam = card::get_attack(d, bc) as i64;
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,bc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(bc)), 1, 0, 0);
        dam
    } else {
        // dam=bc:GetPreviousAttackOnField()
        let dam = card::get_previous_attack_on_field(d, bc) as i64;
        // cd.atk_on_field=dam
        chain::set_effect_data(d, a.e, None, CD_ATK_ON_FIELD, Value::Int(dam));
        dam
    };
    // cd.battled_monster=bc
    chain::set_effect_data(d, a.e, None, CD_BATTLED_MONSTER, Value::Card(bc));
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.damrmop(e,tp,eg,ep,ev,re,r,rp)`
fn damrmop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // local bc=cd.battled_monster
    let bc = chain::effect_data(d, a.e, None, CD_BATTLED_MONSTER).as_card().nil();
    // local battle_relation=bc:IsRelateToBattle()
    let battle_relation = card::is_relate_to_battle(d, bc);
    // local dam=0
    let mut dam: i64 = 0;
    // if battle_relation and bc:IsFaceup() and bc:IsControler(1-tp) then
    if battle_relation && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // dam=bc:GetAttack()
        dam = card::get_attack(d, bc) as i64;
    } else if !battle_relation {
        // dam=cd.atk_on_field
        dam = chain::effect_data(d, a.e, None, CD_ATK_ON_FIELD).int();
    }
    // if Duel.Damage(1-tp,dam,REASON_EFFECT)>0 and battle_relation then
    if duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None) > 0 && battle_relation {
        // Duel.Remove(bc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
