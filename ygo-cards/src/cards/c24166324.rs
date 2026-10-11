//! Meteoroa Drytron
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 24166324;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRYTRON], listed_names: &[22398665], ..ScriptInfo::DEFAULT };

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
    // Add up to 2 of your banished "Drytron" cards to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1, EVENT_REMOVE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg) return eg:IsExists(aux.FaceupFilter(Card.IsSetCard,SET_DRYTRON),1,nil) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetCost(s.thcost)
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Ritual Summon 1 Machine Ritual Monster from your hand or GY
    // local params={filter=s.ritualfilter,lv=Card.GetAttack,matfilter=s.matfilter,location=LOCATION_HAND|LOCATION_GRAVE,requirementfunc=Card.GetAttack}
    let params = ritual::RitualProc {
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| ritualfilter(d, c))),
        lv: Some(ritual::Lv::F(Arc::new(|d: &mut Duel, c: CardId| card::get_attack(d, c) as i64))),
        matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| matfilter(d, c)))),
        location: Some(LOCATION_HAND | LOCATION_GRAVE),
        requirementfunc: Some(Arc::new(|d: &mut Duel, c: CardId, _rc: CardId| card::get_attack(d, c) as i64)),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.ritualcost)
    effect::set_cost(d, e2, Some(cost_target(ritualcost)));
    // e2:SetTarget(Ritual.Target(params))
    let tg = ritual::target(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Ritual.Operation(params))
    let op = ritual::operation(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg) return eg:IsExists(aux.FaceupFilter(Card.IsSetCard,SET_DRYTRON),1,nil) end
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DRYTRON as u16]), 1, NO_EX)
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,aux.FilterBoolFunction(Card.IsSetCard,SET_DRYTRON),1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_DRYTRON as u16])), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,aux.FilterBoolFunction(Card.IsSetCard,SET_DRYTRON),1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_DRYTRON as u16])), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRYTRON) and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DRYTRON as u16]) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_REMOVED) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,2,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_REMOVED, 0, 1, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
// function s.ritualfilter(c)
fn ritualfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetAttack()>0 and c:IsRitualMonster() and c:IsRace(RACE_MACHINE)
    card::get_attack(d, c) > 0 && card::is_ritual_monster(d, c) && card::is_race(d, c, RACE_MACHINE)
}
// function s.matfilter(c)
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:GetAttack()>0
    card::is_race(d, c, RACE_MACHINE) && card::get_attack(d, c) > 0
}
// function s.ritualcostfilter(c)
fn ritualcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(22398665) and not c:IsPublic()
    card::is_code(d, c, &[22398665]) && !card::is_public(d, c)
}
// function s.ritualcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn ritualcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ritualcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ritualcostfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ritualcostfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&ritualcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
