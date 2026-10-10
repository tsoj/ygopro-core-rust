//! Monster Gate
use super::prelude::*;

pub const ID: u32 = 43040603;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return (e:GetLabel()==1 or Duel.GetLocationCount(tp,LOCATION_MZONE)>0) and Duel.IsPlayerCanSpecialSummon(tp)
    //     and Duel.IsExistingMatchingCard(Card.IsSummonableCard,tp,LOCATION_DECK,0,1,nil) and Duel.IsPlayerCanDiscardDeck(tp,1)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_EHERO_BLAZEMAN) end
    if a.chk == Some(0) {
        return (effect::get_label(d, a.e) == 1 || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_player_can_special_summon(d, a.tp, None)
            && duel::is_existing_matching_card(d, Some(&card::is_summonable_card), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_player_can_discard_deck(d, a.tp, 1)
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
    // if not Duel.IsPlayerCanSpecialSummon(tp) or not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_special_summon(d, a.tp, None) || !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSummonableCard,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&card::is_summonable_card), a.tp, LOCATION_DECK, 0, NO_EX);
    // local dcount=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let dcount = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) as i64;
    // local seq=-1
    let mut seq: i64 = -1;
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // local spcard=nil
    let mut spcard: Option<CardId> = None;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // if tc:GetSequence()>seq then
        let s = card::get_sequence(d, tc) as i64;
        if s > seq {
            // seq=tc:GetSequence()
            seq = s;
            // spcard=tc
            spcard = Some(tc);
        }
    }
    // if seq==-1 then
    if seq == -1 {
        // Duel.ConfirmDecktop(tp,dcount)
        duel::confirm_decktop(d, a.tp, dcount as u32);
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // return
        return;
    }
    // Duel.ConfirmDecktop(tp,dcount-seq)
    duel::confirm_decktop(d, a.tp, (dcount - seq) as u32);
    let spcard = spcard.nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and spcard:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, spcard, a.e, 0, a.tp, false, false, None, None, None)
    {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if dcount-seq==1 then Duel.SpecialSummon(spcard,0,tp,tp,false,false,POS_FACEUP)
        if dcount - seq == 1 {
            duel::special_summon(d, spcard, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        } else {
            // Duel.SpecialSummonStep(spcard,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, spcard, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // Duel.DiscardDeck(tp,dcount-seq-1,REASON_EFFECT)
            duel::discard_deck(d, a.tp, (dcount - seq - 1) as u16, REASON_EFFECT);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    } else {
        // Duel.DiscardDeck(tp,dcount-seq,REASON_EFFECT)
        duel::discard_deck(d, a.tp, (dcount - seq) as u16, REASON_EFFECT);
    }
}
