//! The True Name
use super::prelude::*;

pub const ID: u32 = 39913299;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1)
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, tp, 1)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac = duel::announce_card(d, tp, &[TYPE_EXTRA as u64, OPCODE_ISTYPE as u64, OPCODE_NOT as u64]);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thspfilter(c,e,tp,ft)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DIVINE) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_attribute(d, c, ATTRIBUTE_DIVINE)
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let dt = duel::get_decktop_group(d, tp, 1);
    let tc = group::get_first(d, &dt).nil();
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if tc:IsCode(ac) then
    if card::is_code(d, tc, &[ac as u32]) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if not tc:IsAbleToHand() then return Duel.SendtoGrave(tc,REASON_RULE) end
        if !card::is_able_to_hand(d, tc, None) {
            duel::send_to_grave(d, tc, REASON_RULE, None, None);
            return;
        }
        // if not (Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND)) then return end
        if !(duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_HAND)) {
            return;
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // local g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_DECK,0,nil,e,tp,ft)
        let g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, e, tp, ft)), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sel) else {
            // if not sc then return end
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // aux.ToHandOrElse(sc,tp, function(sc) return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //   function(sc) return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, c| ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
            Some(&|d, sc: &CardSet| duel::special_summon(d, sc.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 3)),
        );
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
    } else {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
