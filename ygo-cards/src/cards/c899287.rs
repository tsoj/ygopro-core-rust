//! Glife the Phantom Bird
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 899287;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STROMBERG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_STROMBERG) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_STROMBERG]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetFirstMatchingCard(s.thfilter,tp,LOCATION_DECK,0,nil)
    let tg = duel::get_first_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if tg then
    if let Some(tg) = tg {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tg), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tg)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tg));
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.filter(chkc) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
