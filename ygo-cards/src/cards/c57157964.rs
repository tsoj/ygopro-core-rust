//! Fossil Machine Skull Convoy
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 57157964;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FOSSIL_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Susion summon procedure
    // Fusion.AddProcMix(c,true,true,s.ffilter,aux.FilterBoolFunctionEx(Card.IsLevelAbove,7))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, sumtype, tp| ffilter(d, c, fc, sumtype, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _sumtype, _tp| card::is_level_above(d, c, 7))),
        ],
    );
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Clock Lizard check
    // Auxiliary.addLizardCheck(c)
    let _ = aux::add_lizard_check(d, c);
    // Must first be special summoned with "Fossil Fusion"
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetValue(aux.FossilLimit)
    effect::set_value_fn(d, e0, aux::fossil_limit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Opponent's monsters loses ATK equal to their original DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can make 3 attacks on monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
    // e2:SetValue(2)
    effect::set_value(d, e2, 2);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 1000 damage after destroying a monster by battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCondition(aux.bdocon)
    effect::set_condition(d, e3, Some(aux::bdocon()));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`: "Check for a rock monster in your GY"
fn ffilter(d: &mut Duel, c: CardId, _fc: CardId, _sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_ROCK) and c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp)
    card::is_race(d, c, RACE_ROCK) && card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, tp)
}
/// `s.condition(e)`: "If fusion summoned"
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let handler = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, handler)
}
/// `s.atkval(e,c)`: "Get original DEF"
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local val=math.max(c:GetBaseDefense(),0)
    let val = card::get_base_defense(d, c).max(0);
    // return val*-1
    (val * -1) as i64
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`: "Activation legality"
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
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, (1 - a.tp) as u8, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`: "Inflict 1000 damage after destroying a monster by battle"
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
