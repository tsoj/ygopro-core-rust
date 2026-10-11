//! セイバー・コンビネーション
//! Saber Combination
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 35240714;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_X_SABER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While you have 10 or more "X-Saber" monsters in your field, GY, and/or banishment, "X-Saber" monsters you control gain ATK equal to their own original DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(lua_fn(atkcon)));
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_X_SABER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_X_SABER as u16]))));
    // e1:SetValue(function(e,c) return c:GetBaseDefense() end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_base_defense(d, c) as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "X-Saber" monster from your hand or Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.handdeckspcon)
    effect::set_condition(d, e2, Some(condition(handdeckspcon)));
    // e2:SetTarget(s.sptg(LOCATION_HAND|LOCATION_DECK))
    effect::set_target(d, e2, Some(cost_target(|d, a| sptg(d, a, LOCATION_HAND | LOCATION_DECK))));
    // e2:SetOperation(s.spop(LOCATION_HAND|LOCATION_DECK))
    effect::set_operation(d, e2, Some(operation(|d, a| spop(d, a, LOCATION_HAND | LOCATION_DECK))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "X-Saber" monster from your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e3, Some(lua_fn(e3cond)));
    // e3:SetTarget(s.sptg(LOCATION_HAND))
    effect::set_target(d, e3, Some(cost_target(|d, a| sptg(d, a, LOCATION_HAND))));
    // e3:SetOperation(s.spop(LOCATION_HAND))
    effect::set_operation(d, e3, Some(operation(|d, a| spop(d, a, LOCATION_HAND))));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_X_SABER) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_X_SABER as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.atkfilter,e:GetHandlerPlayer(),LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED,0,10,nil)
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED, 0, 10, NO_EX)
}
/// `s.handdeckspconfilter(c,tp)`
fn handdeckspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsSetCard(SET_X_SABER) and c:IsSummonLocation(LOCATION_EXTRA) and c:IsFaceup()
    card::is_summon_player(d, c, tp)
        && card::is_set_card(d, c, &[SET_X_SABER as u16])
        && card::is_summon_location(d, c, LOCATION_EXTRA)
        && card::is_faceup(d, c)
}
// function s.handdeckspcon(e,tp,eg,ep,ev,re,r,rp)
fn handdeckspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.handdeckspconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| handdeckspconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_X_SABER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_X_SABER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(location)  -- returns function(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs, location: u32) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,location,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, location, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,location)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, location as i32);
    true
}
// function s.spop(location)  -- returns function(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs, location: u32) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,location,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, location, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end
fn e3cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let ac = duel::get_attacker(d).nil();
    card::is_controler(d, ac, 1 - tp)
}
