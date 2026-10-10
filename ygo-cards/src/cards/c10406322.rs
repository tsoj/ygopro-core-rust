//! Alsei, the Sylvan High Protector
use super::prelude::*;

pub const ID: u32 = 10406322;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Declare 1 card name and excavate the top card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return target to top or bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    // (only set and read by this function, so it is a local)
    let announce_filter = [TYPE_EXTRA as u64, OPCODE_ISTYPE, OPCODE_NOT];
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // if tc:IsCode(ac) and tc:IsAbleToHand() then
    if card::is_code(d, tc, &[ac]) && card::is_able_to_hand(d, tc, None) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_DECK) and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_DECK) && card::is_previous_controler(d, c, tp)
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.tdtg(e,tp,...,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if tc:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ) or Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))==0 then
    if card::is_type(d, tc, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ) || duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]) == 0 {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    } else {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
