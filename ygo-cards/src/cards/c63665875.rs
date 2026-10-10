//! Goblin Zombie
use super::prelude::*;

pub const ID: u32 = 63665875;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // handes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.ddcon)
    effect::set_condition(d, e1, Some(condition(ddcon)));
    // e1:SetTarget(s.ddtg)
    effect::set_target(d, e1, Some(cost_target(ddtg)));
    // e1:SetOperation(s.ddop)
    effect::set_operation(d, e1, Some(operation(ddop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.srcon)
    effect::set_condition(d, e2, Some(condition(srcon)));
    // e2:SetTarget(s.srtg)
    effect::set_target(d, e2, Some(cost_target(srtg)));
    // e2:SetOperation(s.srop)
    effect::set_operation(d, e2, Some(operation(srop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.ddcon(e,tp,eg,ep,ev,re,r,rp)
fn ddcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// s.ddtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 1);
    true
}
/// s.ddop(e,tp,eg,ep,ev,re,r,rp)
fn ddop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(1-tp,1,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 1, REASON_EFFECT);
}
/// s.srcon(e,tp,eg,ep,ev,re,r,rp)
fn srcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// s.srtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn srtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDefenseBelow(1200) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToHand()
    card::is_defense_below(d, c, 1200) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_hand(d, c, None)
}
/// s.srop(e,tp,eg,ep,ev,re,r,rp)
fn srop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
