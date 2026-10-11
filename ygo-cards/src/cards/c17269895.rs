//! King's Resonance
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 17269895;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply these effects, in sequence, based on the number of monsters your opponent Special Summoned this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of how many monsters each player has Special Summoned in a given turn
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    // local g0,g1=eg:Split(Card.IsSummonPlayer,nil,0)
    let (g0, g1) = group::split(d, &eg, &|d, c| card::is_summon_player(d, c, 0), NO_EX);
    // for sc0 in g0:Iter() do
    for _sc0 in group::iter(d, &g0) {
        // Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    // for sc1 in g1:Iter() do
    for _sc1 in group::iter(d, &g1) {
        // Duel.RegisterFlagEffect(1,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 1, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsSynchroMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_synchro_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_faceup(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetFlagEffect(1-tp,id)
    let ct = duel::get_flag_effect(d, 1 - tp, ID);
    // if chk==0 then
    if a.chk == Some(0) {
        // local b1=ct>=1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp)
        let b1 = ct >= 1
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
        // local b2=ct>=3
        let b2 = ct >= 3;
        // local b3=ct>=5 and Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,nil)
        let b3 = ct >= 5 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
        // return b1 or b2 or b3
        return b1 || b2 || b3;
    }
    // if ct>=1 then
    if ct >= 1 {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    }
    // if ct>=5 then
    if ct >= 5 {
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - tp, LOCATION_MZONE as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,1-tp,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - tp, POS_FACEDOWN_DEFENSE as i32);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and ct>=10 then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && ct >= 10 {
        // Duel.SetChainLimit(aux.FALSE)
        duel::set_chain_limit(d, aux_false());
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetFlagEffect(1-tp,id)
    let ct = duel::get_flag_effect(d, 1 - tp, ID);
    // local breakeff=false
    let mut breakeff = false;
    // if ct>=1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if ct >= 1 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
            tp,
            LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // breakeff=true
        breakeff = true;
    }
    // if ct>=3 then
    if ct >= 3 {
        // if breakeff then Duel.BreakEffect() end
        if breakeff {
            duel::break_effect(d);
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local reset_count=Duel.IsTurnPlayer(tp) and 2 or 1
        let reset_count: u16 = if duel::is_turn_player(d, tp) { 2 } else { 1 };
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_count)
        aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), RESET_PHASE | PHASE_END | RESET_SELF_TURN, Some(reset_count));
        // "Red Dragon Archfiend" in your Monster Zone is unaffected by your opponent's activated effects until the end of your next turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(aux.TargetBoolFunction(Card.IsCode,CARD_RED_DRAGON_ARCHFIEND))
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]))));
        // e1:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end)
        effect::set_value_fn(d, e1, lua_fn(immune_value));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_count)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_count);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // breakeff=true
        breakeff = true;
    }
    // if ct>=5 then
    if ct >= 5 {
        // if breakeff then Duel.BreakEffect() end
        if breakeff {
            duel::break_effect(d);
        }
        // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
// function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end
fn immune_value(d: &mut Duel, a: Args) -> bool {
    // te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated()
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
