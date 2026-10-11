//! 魔救の調律者
use super::prelude::*;

pub const ID: u32 = 24925387;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can place 1 other "Adamancipator" card from your hand on top of the Deck; Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can excavate the top 5 cards of your Deck, and if you do, you can Special Summon 1 excavated Level 4 or lower Rock monster, also place the rest on the bottom of the Deck in any order
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.excavtg)
    effect::set_target(d, e2, Some(cost_target(excavtg)));
    // e2:SetOperation(s.excavop)
    effect::set_operation(d, e2, Some(operation(excavop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ADAMANCIPATOR) and c:IsAbleToDeckAsCost()
    card::is_set_card(d, c, &[SET_ADAMANCIPATOR as u16]) && card::is_able_to_deck_as_cost(d, c)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&spcostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, &g, None, SEQ_DECKTOP as i32, REASON_COST, None);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=5 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= 5;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn excavspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsLevelBelow(4) and c:IsRace(RACE_ROCK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_ROCK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // local excavg=Duel.GetDecktopGroup(tp,5)
    let excavg = duel::get_decktop_group(d, tp, 5);
    // local remaining_count=#excavg
    let mut remaining_count = excavg.len() as u32;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // local g=excavg:Match(s.excavspfilter,nil,e,tp)
        let mut g = excavg.clone();
        group::match_(d, &mut g, &|d, c| excavspfilter(d, c, e, tp), NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.DisableShuffleCheck()
                duel::disable_shuffle_check(d, true);
                // remaining_count=remaining_count-1
                remaining_count -= 1;
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
    // if remaining_count>0 then
    if remaining_count > 0 {
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>remaining_count then
        if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > remaining_count as i32 {
            // Duel.MoveToDeckBottom(remaining_count,tp)
            duel::move_decktop_to_bottom(d, remaining_count, tp);
        }
        // Duel.SortDeckbottom(tp,tp,remaining_count)
        duel::sort_deckbottom(d, tp, tp, remaining_count as u16);
    }
}
