//! Cyberse Wicckid
use super::prelude::*;

pub const ID: u32 = 52698008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Cannot be destroyed by card effect
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Cyberse monsters cannot be destroyed by card effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.indtg)
    effect::set_target(d, e3, Some(effect_card_fn(indtg)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Search Cyberse tuner, ignition effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetCost(s.thcost)
    effect::set_cost(d, e4, Some(cost_target(thcost)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.indcon(e)`: check if it was link summoned
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // return c:IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.indtg(e,c)`: linked Cyberse monsters
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and e:GetHandler():GetLinkedGroup():IsContains(c)
    if !card::is_race(d, c, RACE_CYBERSE) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    group::is_contains(&lg, c)
}
/// `s.thcfilter(c,tp,lg)`: a monster special summoned to its zones
fn thcfilter(d: &mut Duel, c: CardId, tp: u8, lg: &CardSet) -> bool {
    // return c:IsControler(tp) and lg:IsContains(c)
    card::is_controler(d, c, tp) && group::is_contains(lg, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    // return eg:IsExists(s.thcfilter,1,nil,tp,lg)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thcfilter(d, c, tp, &lg), 1, NO_EX)
}
/// `s.cfilter(c)`: a Cyberse monster that can be banished as a cost
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_CYBERSE) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`: banish 1 Cyberse monster from the GY
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.thfilter(c,tp)`: a Cyberse Tuner that can be added to hand
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsType(TYPE_TUNER) and c:IsAbleToHand()
    card::is_race(d, c, RACE_CYBERSE) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`: activation legality
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`: add the tuner to hand
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
