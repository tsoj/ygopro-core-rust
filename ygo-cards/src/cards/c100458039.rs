//! Xenovader△ Territory - Valvols
//! 異解△領域－ヴァルヴォルス
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 100458039;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_XENOVADER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this card by banishing (face-down) 5 cards from your GY and/or the top of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can add 1 of your face-down banished "Xenovader△" cards to your hand, except "Xenovader△ Territory - Valvols". You can only use this effect of "Xenovader△ Territory - Valvols" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While you control a Level 5 or higher "Xenovader△" monster and you have no cards in your Deck, your opponent cannot activate cards or effects during your turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetDecktopGroup(tp,5)
    let dg = duel::get_decktop_group(d, a.tp, 5);
    // local dc=dg:FilterCount(Card.IsAbleToRemoveAsCost,nil,POS_FACEDOWN)
    let dc = group::filter_count(d, &dg, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), NO_EX) as i64;
    // local gyct=Duel.GetMatchingGroupCount(Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,nil,POS_FACEDOWN)
    let gyct = duel::get_matching_group_count(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // if chk==0 then return dc==5 or gyct+dc>=5 end
    if a.chk == Some(0) {
        return dc == 5 || gyct + dc >= 5;
    }
    // local rg=Group.CreateGroup()
    let mut rg = group::create_group(&[]);
    // if dc<5 or (gyct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if dc < 5 || (gyct > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // rg=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_GRAVE,0,math.max(5-dc,1),5,nil,POS_FACEDOWN)
        rg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8))), a.tp, LOCATION_GRAVE, 0, (5 - dc).max(1) as u16, 5, false, NO_EX).nil();
        // Duel.HintSelection(rg)
        duel::hint_selection(d, rg.clone(), true);
    }
    // if #rg<5 then
    if rg.len() < 5 {
        // rg=rg+Duel.GetDecktopGroup(tp,5-#rg)
        let deck = duel::get_decktop_group(d, a.tp, (5 - rg.len()) as u32);
        rg = group::union(&rg, &deck);
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(rg,POS_FACEDOWN,REASON_COST)
    duel::remove(d, rg, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}

/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsSetCard(SET_XENOVADER) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_facedown(d, c) && card::is_set_card(d, c, &[SET_XENOVADER as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}

/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}

/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}

/// `s.actfilter(c)`
fn actfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_XENOVADER) and c:IsFaceup()
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_XENOVADER as u16]) && card::is_faceup(d, c)
}

/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsTurnPlayer(tp) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0
    //     and Duel.IsExistingMatchingCard(s.actfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_turn_player(d, tp) && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 && duel::is_existing_matching_card(d, Some(&actfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
