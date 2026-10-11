//! Sinful Spoils Struggle
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 67457739;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SINFUL_SPOILS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target any number of cards on the field (the max. number of targets on each side is the number of that player's "Sinful Spoils" cards that are banished or in their GY); destroy them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Set card is destroyed or banished by your opponent's activated effect: You can shuffle up to 2 cards from the field into the Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2a, CATEGORY_TODECK);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2a, EVENT_DESTROYED);
    // e2a:SetCondition(s.tdcon)
    effect::set_condition(d, e2a, Some(condition(tdcon)));
    // e2a:SetTarget(s.tdtg)
    effect::set_target(d, e2a, Some(cost_target(tdtg)));
    // e2a:SetOperation(s.tdop)
    effect::set_operation(d, e2a, Some(operation(tdop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2b, EVENT_REMOVE);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}

/// `s.desrescon(max0,max1)`: the returned rescon, `function(sg,e,tp,mg)`
fn desrescon(d: &mut Duel, sg: &CardSet, tp: u8, max0: usize, max1: usize) -> (bool, bool) {
    // local ct0=sg:FilterCount(Card.IsControler,nil,tp)
    let ct0 = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // local res=ct0<=max0 and (#sg-ct0)<=max1
    let res = ct0 <= max0 && (sg.len() - ct0) <= max1;
    // return res,not res
    (res, !res)
}

/// `s.sinful_faceup`: `aux.FaceupFilter(Card.IsSetCard,SET_SINFUL_SPOILS)`
fn sinful_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SINFUL_SPOILS as u16])
}

/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    let tp = a.tp;
    // local c=e:GetHandler() (for the e:GetHandler() below)
    let c = effect::get_handler(d, a.e).nil();
    // local ct0=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_SINFUL_SPOILS),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let ct0 = duel::get_matching_group_count(d, Some(&sinful_faceup), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // local ct1=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_SINFUL_SPOILS),tp,0,LOCATION_GRAVE|LOCATION_REMOVED,nil)
    let ct1 = duel::get_matching_group_count(d, Some(&sinful_faceup), tp, 0, LOCATION_GRAVE | LOCATION_REMOVED, NO_EX);
    // local g0=Duel.GetTargetGroup(nil,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let g0 = duel::get_target_group(d, None, tp, LOCATION_ONFIELD, 0, c.into());
    // local g1=Duel.GetTargetGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g1 = duel::get_target_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return (#g0>0 and ct0>0) or (#g1>0 and ct1>0) end
    if a.chk == Some(0) {
        return (!g0.is_empty() && ct0 > 0) || (!g1.is_empty() && ct1 > 0);
    }
    // local tg=aux.SelectUnselectGroup(g0+g1,e,tp,1,ct0+ct1,s.desrescon(ct0,ct1),1,tp,HINTMSG_DESTROY)
    let mut all = g0.clone();
    group::add_card(&mut all, &g1);
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp2: u8, _mg: &CardSet, _c: Option<CardId>| desrescon(d, sg, tp2, ct0, ct1);
    let tg = aux::select_unselect_group(d, &all, Some(a.e), tp, Some(1), Some(ct0 + ct1), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), n, tp, 0);
    true
}

/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then Duel.Destroy(tg,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}

/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEDOWN) and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    //   and rp==1-tp and re and re:IsActivated() and c:IsReason(REASON_EFFECT)
    card::is_previous_position(d, c, POS_FACEDOWN as u32)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, a.tp)
        && a.rp == 1 - a.tp
        && a.re.is_some_and(|re| effect::is_activated(d, re))
        && card::is_reason(d, c, REASON_EFFECT)
}

/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}

/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,2,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 2, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
