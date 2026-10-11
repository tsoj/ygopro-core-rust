//! Born from Draconis
//! ボーン・フロム・ドラコニス
use super::prelude::*;

pub const ID: u32 = 96699830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON as u32]).first().copied().unwrap_or(0) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(a.e));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    let h = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se~=e:GetLabelObject() and c:GetFlagEffect(id)==0
    let e = a.e(0);
    let se = a.v(6).as_effect();
    let label = effect::get_label_object(d, e).as_effect();
    let c = a.c(1);
    se != label && card::get_flag_effect(d, c, ID) == 0
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsFaceup() and c:IsAbleToRemove()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_faceup(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,true,false) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsLevelAbove(6)
    card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && card::is_level_above(d, c, 6)
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5
    card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // local gg=not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
        //     or Group.CreateGroup()
        let gg = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
            duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, NO_EX)
        } else {
            CardSet::new()
        };
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // return ((#mg>0 and mg:FilterCount(s.mzfilter,nil)+ft>0) or (#gg>0 and ft>0))
        //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp)
        let mg_ok = !mg.is_empty() && group::filter_count(d, &mg, &mzfilter, NO_EX) as i32 + ft > 0;
        let gg_ok = !gg.is_empty() && ft > 0;
        return (mg_ok || gg_ok) && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // local g
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    let g = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
        duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX)
    } else {
        // g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
        duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX)
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetParam(#g)
    duel::set_target_param(d, g.len() as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]).first().map_or(0, |v| v.int());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // e1:SetValue(ct*500)
            effect::set_value(d, e1, ct * 500);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // local e3=Effect.CreateEffect(e:GetHandler())
            let e3 = effect::create_effect(d, h);
            // e3:SetDescription(3100)
            effect::set_description(d, e3, 3100);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
            // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e3:SetRange(LOCATION_MZONE)
            effect::set_range(d, e3, LOCATION_MZONE);
            // e3:SetValue(s.efilter)
            effect::set_value_fn(d, e3, lua_fn(efilter));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler()~=re:GetOwner()
    let h = effect::get_handler(d, a.e(0)).nil();
    let owner = effect::get_owner(d, a.e(1)).nil();
    h != owner
}
