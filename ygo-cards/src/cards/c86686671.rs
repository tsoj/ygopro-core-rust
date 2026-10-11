//! Cyber Repair Plant
use super::prelude::*;

pub const ID: u32 = 86686671;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_CYBER_DRAGON) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_CYBER_DRAGON])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX))),
    );
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()
    card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToDeck()
    card::is_race(d, c, RACE_MACHINE) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_deck(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return e:GetLabel()~=0 and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) != 0 && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b2 = duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b3=b1 and b2 and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,3,nil,CARD_CYBER_DRAGON)
    let b3 = b1 && b2 && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_CYBER_DRAGON])), a.tp, LOCATION_GRAVE, 0, 3, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // local cat=0
    let mut cat: u64 = 0;
    // if op==1 or op==3 then
    if op == 1 || op == 3 {
        // cat=CATEGORY_TOHAND+CATEGORY_SEARCH
        cat = CATEGORY_TOHAND + CATEGORY_SEARCH;
        // e:SetCategory(cat)
        effect::set_category(d, a.e, cat);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    // if op==2 or op==3 then
    if op == 2 || op == 3 {
        // cat=cat|CATEGORY_TODECK
        cat |= CATEGORY_TODECK;
        // e:SetCategory(cat)
        effect::set_category(d, a.e, cat);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, a.tp, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local breakeff=false
    let mut breakeff = false;
    // if op==1 or op==3 then
    if op == 1 || op == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
            // breakeff=true
            breakeff = true;
        }
    }
    // if op==2 or op==3 then
    if op == 2 || op == 3 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // if breakeff then Duel.BreakEffect() end
            if breakeff {
                duel::break_effect(d);
            }
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
