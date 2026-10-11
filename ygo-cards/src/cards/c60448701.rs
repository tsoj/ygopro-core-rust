//! Ogdoadic Origin
//! 溟界の淵源
use super::prelude::*;

pub const ID: u32 = 60448701;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to grave
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.gycon)
    effect::set_condition(d, e2, Some(condition(gycon)));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // mill
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_DECKDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.mlcon)
    effect::set_condition(d, e3, Some(condition(mlcon)));
    // e3:SetTarget(s.mltg)
    effect::set_target(d, e3, Some(cost_target(mltg)));
    // e3:SetOperation(s.mlop)
    effect::set_operation(d, e3, Some(operation(mlop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetPreviousRaceOnField()&RACE_REPTILE==RACE_REPTILE and (c:IsReason(REASON_BATTLE) or (c:GetReasonPlayer()==1-tp and c:IsReason(REASON_EFFECT)))
    //   and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    (card::get_previous_race_on_field(d, c) & RACE_REPTILE) == RACE_REPTILE
        && (card::is_reason(d, c, REASON_BATTLE) || (card::get_reason_player(d, c) == 1 - tp && card::is_reason(d, c, REASON_EFFECT)))
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
}
/// `s.gycon(e,tp,eg,ep,ev,re,r,rp)`
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SendtoGrave(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.mlcon(e,tp,eg,ep,ev,re,r,rp)`
fn mlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_FZONE) and c:IsPreviousControler(tp)
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_FZONE)
        && card::is_previous_controler(d, c, a.tp)
}
/// `Duel.GetMatchingGroup(Card.IsRace,tp,LOCATION_GRAVE,0,nil,RACE_REPTILE):GetClassCount(Card.GetCode)`
fn reptile_class_count(d: &mut Duel, tp: u8) -> i64 {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_race(d, c, RACE_REPTILE)), tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64
}
/// `s.mltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=...
    let ct = reptile_class_count(d, tp);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDiscardDeck(1-tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_discard_deck(d, 1 - tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,1-tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - tp, ct as i32);
    true
}
/// `s.mlop(e,tp,eg,ep,ev,re,r,rp)`
fn mlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=...
    let ct = reptile_class_count(d, tp);
    // if ct>0 and Duel.IsPlayerCanDiscardDeck(1-tp,ct) then Duel.DiscardDeck(1-tp,ct,REASON_EFFECT) end
    if ct > 0 && duel::is_player_can_discard_deck(d, 1 - tp, ct as u32) {
        duel::discard_deck(d, 1 - tp, ct as u16, REASON_EFFECT);
    }
}
