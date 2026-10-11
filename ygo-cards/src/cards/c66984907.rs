//! Amaze Attraction Cyclo-Coaster
use super::prelude::*;

pub const ID: u32 = 66984907;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    // aux.AddAttractionEquipProc(c)
    aux::add_attraction_equip_proc(d, c);
    // You: Target 1 Spell/Trap your opponent controls; send both it and this card to the GY.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.AttractionEquipCon(true))
    effect::set_condition(d, e1, Some(aux::attraction_equip_con(true)));
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent: Add 1 "Amazement" monster from your Deck to your hand, and if you do, send this card to the GY.
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetProperty(0)
    effect::set_property(d, e2, 0, 0);
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // e2:SetCondition(aux.AttractionEquipCon(false))
    effect::set_condition(d, e2, Some(aux::attraction_equip_con(false)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.gyfilter(c)
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGrave()
    card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
// s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and s.gyfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && gyfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.gyfilter,tp,0,LOCATION_ONFIELD,1,nil) and e:GetHandler():IsAbleToGrave() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_target(d, Some(&gyfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && card::is_able_to_grave(d, h, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.gyfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&gyfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsRelateToEffect(e) and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_effect(d, c, a.e) {
            // local g=Group.FromCards(c,tc)
            let mut g = CardSet::from_card(c);
            group::add_card(&mut g, &CardSet::from_card(tc));
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
        }
    }
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMAZEMENT) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_AMAZEMENT as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) and e:GetHandler():IsAbleToGrave() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && card::is_able_to_grave(d, h, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) != 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.SendtoGrave(c,REASON_EFFECT)
            duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        }
    }
}
