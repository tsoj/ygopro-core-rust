//! Vaalmonica Creation
use super::prelude::*;

pub const ID: u32 = 98167225;
const AGATHOKAKOLOGICAL_VOICE: u32 = 39210885;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAALMONICA], counter_place_list: &[COUNTER_RESONANCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP));
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While you have 6 or more Resonance Counters on your field, "Vaalmonica" Link Monsters you control gain 1200 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function(e) return Duel.GetCounter(e:GetHandlerPlayer(),1,0,COUNTER_RESONANCE)>=6 end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetTarget(function(e,c) return c:IsSetCard(SET_VAALMONICA) and c:IsLinkMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_link_monster(d, c))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(1200)
    effect::set_value(d, e1, 1200);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent Special Summons a monster(s), you can: Immediately after this effect resolves, Link Summon 1 "Vaalmonica" Link Monster
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
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.lktg)
    effect::set_target(d, e2, Some(cost_target(lktg)));
    // e2:SetOperation(s.lkop)
    effect::set_operation(d, e2, Some(operation(lkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is sent from the hand or field to the GY: You can place Resonance Counters on 1 card in your Pendulum Zone that you can place a Resonance Counter on, so it has 3 Resonance Counters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.cttg)
    effect::set_target(d, e3, Some(cost_target(cttg)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function(e,tp,eg) (the counter condition of e1)
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_counter(d, p, 1, 0, COUNTER_RESONANCE as u16) >= 6
}
// function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let eg = a.g(2);
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
// function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) end
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD)
}
// function s.lkfilter(c)
fn lkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsLinkSummonable()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_link_summonable(d, c, None, None, 0, 0)
}
// function s.lktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.lkfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&lkfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.lkop(e,tp,eg,ep,ev,re,r,rp)
fn lkop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.lkfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&lkfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // Duel.LinkSummon(tp,sc)
        duel::link_summon(d, a.tp, sc, None, None, 0, 0);
    }
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // local ct=c:GetCounter(COUNTER_RESONANCE)
    let ct = card::get_counter(d, c, COUNTER_RESONANCE as u16);
    // return ct<3 and c:IsCanAddCounter(COUNTER_RESONANCE,3-ct)
    ct < 3 && card::is_can_add_counter(d, c, COUNTER_RESONANCE as u16, (3 - ct) as u16, false, 0)
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_PZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.ctfilter,tp,LOCATION_PZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&ctfilter), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and tc:AddCounter(COUNTER_RESONANCE,3-tc:GetCounter(COUNTER_RESONANCE),true) then
    if let Some(tc) = tc {
        let ct = card::get_counter(d, tc, COUNTER_RESONANCE as u16);
        if card::add_counter(d, tc, COUNTER_RESONANCE as u16, (3 - ct) as u16, true) {
            // Duel.RaiseEvent(tc,EVENT_CUSTOM+AGATHOKAKOLOGICAL_VOICE,e,0,tp,tp,1)
            duel::raise_event(d, tc, EVENT_CUSTOM + AGATHOKAKOLOGICAL_VOICE, Some(a.e), 0, a.tp, a.tp, 1);
        }
    }
}
