//! Reasoning
use super::prelude::*;

pub const ID: u32 = 58577036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummon(tp) and Duel.IsPlayerCanDiscardDeck(tp,1)
    //     and Duel.IsExistingMatchingCard(Card.IsSummonableCard,tp,LOCATION_DECK,0,1,nil)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_EHERO_BLAZEMAN) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon(d, a.tp, None)
            && duel::is_player_can_discard_deck(d, a.tp, 1)
            && duel::is_existing_matching_card(d, Some(&card::is_summonable_card), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, a.tp, CARD_EHERO_BLAZEMAN).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerCanSpecialSummon(tp) or not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_special_summon(d, tp, None) || !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceLevel(1-tp)
    let lv = duel::announce_number_range(d, 1 - tp, None, None, &[]).nil().0 as i64;
    // local g=Duel.GetMatchingGroup(Card.IsSummonableCard,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_summonable_card), tp, LOCATION_DECK, 0, NO_EX);
    // local dcount=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let dcount = duel::get_field_group_count(d, tp, LOCATION_DECK, 0) as i64;
    // local seq=-1
    // local spcard=nil
    let mut seq: i64 = -1;
    let mut spcard: Option<CardId> = None;
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if tc:GetSequence()>seq then
        if card::get_sequence(d, tc) as i64 > seq {
            // seq=tc:GetSequence()
            seq = card::get_sequence(d, tc) as i64;
            // spcard=tc
            spcard = Some(tc);
        }
    }
    // if seq==-1 then
    if seq == -1 {
        // Duel.ConfirmDecktop(tp,dcount)
        duel::confirm_decktop(d, tp, dcount as u32);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // return
        return;
    }
    // Duel.ConfirmDecktop(tp,dcount-seq)
    duel::confirm_decktop(d, tp, (dcount - seq) as u32);
    let spcard = spcard.nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and lv~=spcard:GetLevel()
    //     and spcard:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && lv != card::get_level(d, spcard) as i64
        && card::is_can_be_special_summoned(d, spcard, a.e, 0, tp, false, false, None, None, None)
    {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if dcount-seq==1 then Duel.SpecialSummon(spcard,0,tp,tp,false,false,POS_FACEUP)
        if dcount - seq == 1 {
            duel::special_summon(d, spcard, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.SpecialSummonStep(spcard,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, spcard, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // Duel.DiscardDeck(tp,dcount-seq-1,REASON_EFFECT)
            duel::discard_deck(d, tp, (dcount - seq - 1) as u16, REASON_EFFECT);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    } else {
        // Duel.DiscardDeck(tp,dcount-seq,REASON_EFFECT|REASON_EXCAVATE)
        duel::discard_deck(d, tp, (dcount - seq) as u16, REASON_EFFECT | REASON_EXCAVATE);
    }
}
