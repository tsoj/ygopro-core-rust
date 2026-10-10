//! Ace★Spades Speculation
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 10796448;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Can only Special Summon "A★Speculation(s)" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Fusion Summon procedure
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsAttackAbove,2500),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| card::is_attack_above(d, c, 2500))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| matfilter(d, c))),
        ],
    );
    // Special Summon this card from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains ATK equal to the highest original ATK among the opponent's monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():IsAttackPos() end)
    effect::set_condition(d, e2, Some(condition(atk_cond)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by battle or card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():IsDefensePos() end)
    effect::set_condition(d, e3, Some(condition(def_cond)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEDOWN_DEFENSE) and c:IsLocation(LOCATION_MZONE) and c:IsDefenseBelow(2500)
    card::is_position(d, c, POS_FACEDOWN_DEFENSE as u32) && card::is_location(d, c, LOCATION_MZONE) && card::is_defense_below(d, c, 2500)
}
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0
    //     and sg:FilterCount(Card.IsAttackPos,nil)==1
    //     and sg:FilterCount(Card.IsPosition,nil,POS_FACEDOWN_DEFENSE)==1
    let res = duel::get_m_zone_count(d, tp, sg.into(), Some(tp), None, None).0 > 0
        && group::filter_count(d, sg, &|d, c| card::is_attack_pos(d, c), NO_EX) == 1
        && group::filter_count(d, sg, &|d, c| card::is_position(d, c, POS_FACEDOWN_DEFENSE as u32), NO_EX) == 1;
    (res, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,false,s.spcheck,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, None, 2, 2, false, Some(&spcheck), NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,s.spcheck,nil)
    let sg = duel::select_release_group_cost(d, tp, None, 2, 2, false, Some(&spcheck), NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, &sg, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e) return e:GetHandler():IsAttackPos() end`
fn atk_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
/// `function(e) return e:GetHandler():IsDefensePos() end`
fn def_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,e:GetHandlerPlayer(),0,LOCATION_MZONE,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), p, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return 0 end
    if g.is_empty() {
        return 0;
    }
    // local _,atk=g:GetMaxGroup(Card.GetBaseAttack)
    // return atk
    match group::get_max_group(d, &g, &|d, c| card::get_base_attack(d, c) as i64) {
        Some((_, atk)) => atk,
        None => 0,
    }
}
