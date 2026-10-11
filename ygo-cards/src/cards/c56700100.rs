//! My Friend Purrely
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 56700100;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[56700100],
    listed_series: &[SET_PURRELY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reveal 3 "Purrely" cards and add 1 to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add up to 3 "Purrely" Quick-Play Spells to the hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.rthcond)
    effect::set_condition(d, e3, Some(condition(rthcond)));
    // e3:SetTarget(s.rthtg)
    effect::set_target(d, e3, Some(cost_target(rthtg)));
    // e3:SetOperation(s.rthop)
    effect::set_operation(d, e3, Some(operation(rthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil,SET_PURRELY)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=3 then
    if g.len() >= 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,3,3,nil)
        let sg = group::select(d, &g, tp, 3, 3, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // local tg=sg:RandomSelect(1-tp,1)
        let tg = group::random_select(d, &sg, 1 - tp, 1);
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &tg, None, REASON_EFFECT, None);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
    }
}
// function s.xyzfilter(c,tp)
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsType(TYPE_XYZ) and c:IsPreviousSetCard(SET_PURRELY)
    //     and c:IsPreviousControler(tp) and c:GetReasonPlayer()==1-tp
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_previous_set_card(d, c, &[SET_PURRELY as u16])
        && card::is_previous_controler(d, c, tp)
        && card::get_reason_player(d, c) == 1 - tp
}
// function s.rthcond(e,tp,eg,ep,ev,re,r,rp)
fn rthcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.xyzfilter,1,nil,tp)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| xyzfilter(d, c, tp), 1, NO_EX)
}
// function s.qpfilter(c)
fn qpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsSpell() and c:IsType(TYPE_QUICKPLAY) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_spell(d, c) && card::is_type(d, c, TYPE_QUICKPLAY) && card::is_able_to_hand(d, c, None)
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.qpfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&qpfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.GetMatchingGroup(s.qpfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&qpfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,3,aux.dncheck,1,tp,HINTMSG_ATOHAND)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
