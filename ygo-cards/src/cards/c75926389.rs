//! A-to-Z Energy Load
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 75926389;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_card_types: &[TYPE_UNION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e1, Some(condition(chain_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle up to 6 of your banished LIGHT Machine monsters into the Deck, including a Union monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e2, TIMING_STANDBY_PHASE, None);
    // e2:SetCondition(function(e,tp) return Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(standby_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.drawcountfilter(c)`
fn drawcountfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(8) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsType(TYPE_FUSION) and c:IsFaceup()
    card::is_level_above(d, c, 8) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE) && card::is_type(d, c, TYPE_FUSION) && card::is_faceup(d, c)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end`
fn chain_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.drawcountfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&drawcountfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, tp, ct as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.drawcountfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&drawcountfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // Duel.Draw(p,ct,REASON_EFFECT)
    duel::draw(d, p, ct as u32, REASON_EFFECT);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_MACHINE) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `function(e,tp) return Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp) end`
fn standby_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, a.tp)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil)
        let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, NO_EX);
        // return g:IsExists(Card.IsType,1,nil,TYPE_UNION)
        return group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_UNION), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsType,1,nil,TYPE_UNION)
    (group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_UNION), 1, NO_EX), false)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,6,s.rescon,1,tp,HINTMSG_TODECK,s.rescon)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(6), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), Some(rs), None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
