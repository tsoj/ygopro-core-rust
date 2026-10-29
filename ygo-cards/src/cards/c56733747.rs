//! Elemental HERO Shining Neos Wingman
use super::prelude::*;

pub const ID: u32 = 56733747;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO, SET_NEOS, SET_WINGMAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Materials
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_NEOS,s.ffilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(CARD_NEOS), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))],
    );
    // Must be Fusion Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e0, aux::fuslimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy opponent's cards up to the number of different Attributes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by card effects
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
    // Inflict damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetCondition(aux.bdcon)
    effect::set_condition(d, e4, Some(aux::bdcon()));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.ffilter(c,fc,sumtype,tp)
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_FUSION,fc,sumtype,tp) and c:IsSetCard(SET_WINGMAN,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_WINGMAN as u16], Some(fc), sumtype, Some(tp))
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local attg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let attg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local ct=attg:GetClassCount(Card.GetAttribute)
    let ct = group::get_class_count(d, &attg, &|d, c| card::get_attribute(d, c) as i64);
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return ct>0 and #dg>0 end
    if a.chk == Some(0) {
        return ct > 0 && !dg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local attg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let attg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #attg==0 then return end
    if attg.is_empty() {
        return;
    }
    // local ct=attg:GetClassCount(Card.GetAttribute)
    let ct = group::get_class_count(d, &attg, &|d, c| card::get_attribute(d, c) as i64);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,ct,nil)
        let dg = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
        // if #dg==0 then return end
        if dg.is_empty() {
            return;
        }
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, &dg, true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, &dg, REASON_EFFECT, None, None);
    }
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)*300
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_monster(d, c)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local dam=e:GetHandler():GetBattleTarget():GetBaseAttack()
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    let mut dam = card::get_base_attack(d, bt);
    // if dam<0 then dam=0 end
    if dam < 0 {
        dam = 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
