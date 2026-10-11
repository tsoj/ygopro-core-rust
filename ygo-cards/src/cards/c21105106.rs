//! Nekroz of Sophia
use super::prelude::*;

pub const ID: u32 = 21105106;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEKROZ],
    mat_filter: Some(mat_filter),
    ritual_custom_check: Some(ritual_custom_check),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Must be Ritual Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Prevent Special summons from the Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --remove
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetCost(s.rmcost)
    effect::set_cost(d, e3, Some(cost_target(rmcost)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_HAND) and (st&SUMMON_TYPE_RITUAL)==SUMMON_TYPE_RITUAL
    let h = effect::get_handler(d, a.e(0)).nil();
    let st = a.int(3) as u64;
    card::is_location(d, h, LOCATION_HAND) && (st & SUMMON_TYPE_RITUAL as u64) == SUMMON_TYPE_RITUAL as u64
}
/// `s.mat_filter(c)`
fn mat_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsLocation(LOCATION_ONFIELD)
    card::is_location(d, c, LOCATION_ONFIELD)
}
/// `s.onfieldcontroller(c,tp)`
fn onfieldcontroller(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.ritual_custom_check(e,tp,g,c)`
fn ritual_custom_check(d: &mut Duel, _e: EffectId, tp: u8, g: &CardSet, _c: CardId) -> (bool, bool) {
    // local count=#g
    let count = g.len();
    // local class_count=g:GetClassCount(Card.GetRace)
    let class_count = group::get_class_count(d, g, &|d, c| card::get_race(d, c) as i64);
    // local onfield_self_count=g:FilterCount(s.onfieldcontroller,nil,tp)
    let onfield_self_count = group::filter_count(d, g, &|d, c| onfieldcontroller(d, c, tp), NO_EX);
    // return count==3 and class_count==3 and onfield_self_count==3,count>3 or class_count~=count or onfield_self_count~=count
    (
        count == 3 && class_count == 3 && onfield_self_count == 3,
        count > 3 || class_count != count || onfield_self_count != count,
    )
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEKROZ) and c:IsSpell() and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_spell(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler() (the handler of the effect)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsDiscardable()
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_discardable(d, h, None, None) && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let mut g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SendtoGrave(g,REASON_DISCARD|REASON_COST)
    duel::send_to_grave(d, g, REASON_DISCARD | REASON_COST, None, None);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetReset(RESET_PHASE|PHASE_MAIN1)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN1, 1);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsLocation(LOCATION_EXTRA)
    card::is_location(d, a.c(1), LOCATION_EXTRA)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRitualSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_NORMALSUMMON)==0
    //     and Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==1 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_NORMALSUMMON]).first() == Some(&0) && duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON]).first() == Some(&1);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // --lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,aux.TRUE,RESET_PHASE|PHASE_END,0,0xff)
    aux::add_temp_lizard_check(d, h, a.tp, Some(aux_true()), Some(RESET_PHASE | PHASE_END), Some(0), Some(0xff), None);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e3, EFFECT_CANNOT_MSET);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,false,true))
    card::is_able_to_remove(d, c, None, None, None) && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, false, true))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, h.into());
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
