//! Void Liberation
//! 煉獄の乖放
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 61345801;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL, SET_INFERNOID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Shaddoll" cards from your Deck to your hand of different types (Monster, Spell, or Trap) from the revealed card and each other, then discard 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.Reveal(s.revealfilter,true,1,1,function(e,tp,g) e:SetLabel(g:GetFirst():GetMainCardType()) end))
    effect::set_cost(
        d,
        e1,
        Some(cost::reveal(
            Some(Arc::new(revealfilter)),
            true,
            Some(cost::Count::N(1)),
            Some(cost::Count::N(1)),
            Some(Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet| {
                let tc = group::get_first(d, g).nil();
                let t = card::get_main_card_type(d, tc);
                effect::set_label(d, e, &[t as i64]);
            })),
            None,
        )),
    );
    // e1:SetTarget(s.shaddollthtg)
    effect::set_target(d, e1, Some(cost_target(shaddollthtg)));
    // e1:SetOperation(s.shaddollthop)
    effect::set_operation(d, e1, Some(operation(shaddollthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Infernoid" monster from your Deck or GY your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.infernoidthtg)
    effect::set_target(d, e2, Some(cost_target(infernoidthtg)));
    // e2:SetOperation(s.infernoidthop)
    effect::set_operation(d, e2, Some(operation(infernoidthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.revealfilter(c,e,tp)
fn revealfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and not c:IsPublic()
    //     and Duel.GetMatchingGroup(s.shaddollthfilter,tp,LOCATION_DECK,0,nil,c:GetMainCardType()):GetClassCount(Card.GetMainCardType)==2
    if !(card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && !card::is_public(d, c)) {
        return false;
    }
    let main_type = card::get_main_card_type(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| shaddollthfilter(d, x, main_type)), tp, LOCATION_DECK, 0, NO_EX);
    group::get_class_count(d, &g, &main_type_value) == 2
}
// Card.GetMainCardType as a value function
fn main_type_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_main_card_type(d, c) as i64
}
// function s.shaddollthfilter(c,main_type)
fn shaddollthfilter(d: &mut Duel, c: CardId, main_type: u32) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsAbleToHand() and not c:IsType(main_type)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_able_to_hand(d, c, None) && !card::is_type(d, c, main_type)
}
// function s.shaddollthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn shaddollthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
// function s.shaddollthop(e,tp,eg,ep,ev,re,r,rp)
fn shaddollthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.shaddollthfilter,tp,LOCATION_DECK,0,nil,e:GetLabel())
    let main_type = effect::get_label(d, a.e) as u32;
    let g = duel::get_matching_group(d, Some(&|d, x| shaddollthfilter(d, x, main_type)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetMainCardType),1,tp,HINTMSG_ATOHAND)
    let dp = aux::dpcheck(main_type_value);
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #sg==2 and Duel.SendtoHand(sg,nil,REASON_EFFECT)>0 then
    if sg.len() == 2 && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
// function s.infernoidthfilter(c)
fn infernoidthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNOID) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_INFERNOID as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.infernoidthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn infernoidthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.infernoidthfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&infernoidthfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.infernoidthop(e,tp,eg,ep,ev,re,r,rp)
fn infernoidthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.infernoidthfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let necro = aux::necro_valley_filter(infernoidthfilter);
    let g = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
