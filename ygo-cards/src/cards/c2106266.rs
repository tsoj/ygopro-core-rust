//! Galloping Gaia
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 2106266;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_GAIA_CHAMPION],
    listed_series: &[SET_GAIA_THE_FIERCE_KNIGHT],
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
    // Activation limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(s.thcost1)
    effect::set_cost(d, e3, Some(cost_target(thcost1)));
    // e3:SetTarget(s.thtg1)
    effect::set_target(d, e3, Some(cost_target(thtg1)));
    // e3:SetOperation(s.thop1)
    effect::set_operation(d, e3, Some(operation(thop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCost(s.thcost2)
    effect::set_cost(d, e4, Some(cost_target(thcost2)));
    // e4:SetTarget(s.thtg2)
    effect::set_target(d, e4, Some(cost_target(thtg2)));
    // e4:SetOperation(s.thop2)
    effect::set_operation(d, e4, Some(operation(thop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsBattlePhase() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GAIA_CHAMPION),tp,LOCATION_MZONE,0,1,nil)
    duel::is_battle_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_GAIA_CHAMPION])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.cfilter1(c)
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GAIA_THE_FIERCE_KNIGHT) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_GAIA_THE_FIERCE_KNIGHT as u16]) && card::is_monster(d, c) && !card::is_public(d, c)
}
// function s.thcost1(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter1), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter1,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter1), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
// function s.thfilter1(c)
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsLevel(5) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_level(d, c, &[5]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg1(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop1(e,tp,eg,ep,ev,re,r,rp)
fn thop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter1,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsLevel(5) and not c:IsPublic()
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_level(d, c, &[5]) && !card::is_public(d, c)
}
// function s.thcost2(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter2), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter2,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter2), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
// function s.thfilter2(c)
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GAIA_THE_FIERCE_KNIGHT) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GAIA_THE_FIERCE_KNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter2), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop2(e,tp,eg,ep,ev,re,r,rp)
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter2), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
