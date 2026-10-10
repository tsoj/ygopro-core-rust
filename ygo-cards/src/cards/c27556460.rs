//! Primite Scream
use super::prelude::*;

pub const ID: u32 = 27556460;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PRIMITE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate a Normal or Special Summon and banish that monster(s)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negsumcon)
    effect::set_condition(d, e1, Some(condition(negsumcon)));
    // e1:SetTarget(s.negsumtg)
    effect::set_target(d, e1, Some(cost_target(negsumtg)));
    // e1:SetOperation(s.negsumop)
    effect::set_operation(d, e1, Some(operation(negsumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set this card from the GY but banish it when it leaves the field
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetHintTiming(TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e3, TIMING_STANDBY_PHASE, None);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.setcon)
    effect::set_condition(d, e3, Some(condition(setcon)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.negsumconfilter(c)`
fn negsumconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsType(TYPE_NORMAL) or (c:IsSetCard(SET_PRIMITE) and c:IsLevelAbove(5))) and c:IsFaceup() and not c:IsType(TYPE_TOKEN)
    (card::is_type(d, c, TYPE_NORMAL) || (card::is_set_card(d, c, &[SET_PRIMITE as u16]) && card::is_level_above(d, c, 5)))
        && card::is_faceup(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.negsumcon(e,tp,eg,ep,ev,re,r,rp)`
fn negsumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    //     and Duel.IsExistingMatchingCard(s.negsumconfilter,tp,LOCATION_MZONE,0,1,nil)
    if duel::get_current_chain(d, true) != 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&negsumconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.negsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsAbleToRemove,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
/// `s.negsumop(e,tp,eg,ep,ev,re,r,rp)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, &eg);
    // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, &eg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PRIMITE),tp,LOCATION_MZONE,0,1,nil)
    duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, a.tp) && duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PRIMITE as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() and Duel.SSet(tp,c)>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) && duel::s_set(d, a.tp, c, None, true) > 0 {
        // Banish it when it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
