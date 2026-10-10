//! Archfiend's Oath
//! デーモンの宣告
use super::prelude::*;

pub const ID: u32 = 22796548;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate the top card of your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 1)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    // (only read by this line, so the value is not kept)
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac = duel::announce_card(d, tp, &[TYPE_EXTRA as u64, OPCODE_ISTYPE as u64, OPCODE_NOT as u64]);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if tc:IsCode(ac) and tc:IsAbleToHand() then
    if card::is_code(d, tc, &[ac as u32]) && card::is_able_to_hand(d, tc, None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
