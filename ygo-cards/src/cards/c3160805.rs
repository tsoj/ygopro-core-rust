//! Exchange of Night and Day
use super::prelude::*;

pub const ID: u32 = 3160805;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // instant
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE as u32));
    // e2:SetCost(s.cost2)
    effect::set_cost(d, e2, Some(cost_target(cost2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn tgfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_LIGHT)
    //     and Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil,c:GetLevel())
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && {
            let lv = card::get_level(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| thfilter1(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
fn thfilter1(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:GetLevel()==lv and c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToHand()
    card::get_level(d, c) == lv && card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_hand(d, c, None)
}
fn tgfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_DARK)
    //     and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c:GetLevel())
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && {
            let lv = card::get_level(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| thfilter2(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
fn thfilter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:GetLevel()==lv and c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()
    card::get_level(d, c) == lv && card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsExistingMatchingCard(s.tgfilter1,tp,LOCATION_HAND,0,1,nil,tp)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| tgfilter1(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.tgfilter2,tp,LOCATION_HAND,0,1,nil,tp)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| tgfilter2(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // local op=2
    let mut op = 2;
    // if Duel.GetFlagEffect(tp,id)==0 and (b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::get_flag_effect(d, a.tp, ID) == 0 && (b1 || b2) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        if b1 && b2 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
            op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
        } else if b1 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,2))
            op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
        } else {
            // op=Duel.SelectOption(tp,aux.Stringid(id,3))+1
            op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3)]) + 1;
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==2 or not e:GetHandler():IsRelateToEffect(e) then return end
    if effect::get_label(d, a.e) == 2 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if effect::get_label(d, a.e) == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter1,tp,LOCATION_HAND,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter1(d, x, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if Duel.SendtoGrave(g,REASON_EFFECT)~=0 then
        if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local tg=Duel.SelectMatchingCard(tp,s.thfilter1,tp,LOCATION_DECK,0,1,1,nil,g:GetFirst():GetLevel())
            let lv = card::get_level(d, group::get_first(d, &g).nil());
            let tg = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter1(d, x, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - a.tp, tg);
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter2,tp,LOCATION_HAND,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter2(d, x, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if Duel.SendtoGrave(g,REASON_EFFECT)~=0 then
        if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) != 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local tg=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,g:GetFirst():GetLevel())
            let lv = card::get_level(d, group::get_first(d, &g).nil());
            let tg = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter2(d, x, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - a.tp, tg);
        }
    }
}
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(s.tgfilter1,tp,LOCATION_HAND,0,1,nil,tp)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| tgfilter1(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.tgfilter2,tp,LOCATION_HAND,0,1,nil,tp)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| tgfilter2(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,3))+1
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3)]) + 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
