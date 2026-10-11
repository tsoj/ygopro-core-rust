//! Heartless Drop Off
use super::prelude::*;

pub const ID: u32 = 47439573;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_HANDES);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD|LOCATION_GRAVE)
    card::is_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_location(d, c, LOCATION_ONFIELD | LOCATION_GRAVE)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - tp), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(eg)
    let eg = a.eg.clone().nil();
    duel::set_target_card(d, eg);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and s.cfilter(c,tp)
    card::is_relate_to_effect(d, c, e) && cfilter(d, c, tp)
}
// function s.rmfilter(c,g)
fn rmfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:IsAbleToRemove() and g:IsExists(Card.IsCode,1,nil,c:GetCode())
    let codes = card::get_code(d, c);
    card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, g, &|d, x| card::is_code(d, x, &codes), 1, NO_EX)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local dg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.filter,nil,e,1-tp)
    let chain_cards = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let dg = group::filter(d, &chain_cards, &|d, c| filter(d, c, e, 1 - tp), NO_EX);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, g.clone());
        // local tg=g:Filter(s.rmfilter,nil,dg)
        let tg = group::filter(d, &g, &|d, c| rmfilter(d, c, &dg), NO_EX);
        // if #tg>0 then
        if tg.len() > 0 {
            // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
