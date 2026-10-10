//! Minerva, Lightsworn Maiden
use super::prelude::*;

pub const ID: u32 = 40164421;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // discard deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // discard deck2
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_DECKDES);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.discon2)
    effect::set_condition(d, e3, Some(condition(discon2)));
    // e3:SetTarget(s.distg2)
    effect::set_target(d, e3, Some(cost_target(distg2)));
    // e3:SetOperation(s.disop2)
    effect::set_operation(d, e3, Some(operation(disop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster()
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c)
}
// function s.thfilter(c,lv)
fn thfilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsLevelBelow(lv) and c:IsRace(RACE_DRAGON) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()
    card::is_level_below(d, c, lv as u32) && card::is_race(d, c, RACE_DRAGON) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // local ct=g:GetClassCount(Card.GetCode)
        let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,ct)
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, ct as i64)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetCode)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,ct)
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, ct as i64)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK|LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_DECK | LOCATION_HAND)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 1);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.DiscardDeck(p,d,REASON_EFFECT)
    duel::discard_deck(d, info[0].int() as u8, info[1].int() as u16, REASON_EFFECT);
}
// function s.discon2(e,tp,eg,ep,ev,re,r,rp)
fn discon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
// function s.distg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 2);
    true
}
// function s.disop2(e,tp,eg,ep,ev,re,r,rp)
fn disop2(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,2,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
}
