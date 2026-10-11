//! 魔救の奇石－ティアマイト
use super::prelude::*;

pub const ID: u32 = 51420096;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Special Summoned by an "Adamancipator" card's effect: You can add 1 "Adamancipator" card from your Deck to your hand, except "Adamancipator Crystal - Tiamite", then you can Special Summon 1 Rock monster from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp)
    //     return re and re:IsCardSetcode(SET_ADAMANCIPATOR) and re:IsHasType(EFFECT_TYPE_ACTIONS)
    // end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is in your GY: You can target 1 Rock Synchro Monster in your field or GY; return it to the Extra Deck, and if you do, place this card on top of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsCardSetcode(SET_ADAMANCIPATOR) and re:IsHasType(EFFECT_TYPE_ACTIONS)
    match a.re {
        Some(re) => effect::is_card_setcode(d, re, &[SET_ADAMANCIPATOR as u16]) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIONS),
        None => false,
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ADAMANCIPATOR) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ADAMANCIPATOR as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsRace(RACE_ROCK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ROCK) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 or not g:GetFirst():IsLocation(LOCATION_HAND) then return end
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, &g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    let first = group::get_first(d, &g).nil();
    if !card::is_location(d, first, LOCATION_HAND) {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local hg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let hg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #hg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if hg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local hsg=hg:Select(tp,1,1,nil)
    let hsg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
    // if #hsg>0 then
    if !hsg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(hsg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &hsg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsRace(RACE_ROCK) and c:IsSynchroMonster() and c:IsFaceup() and c:IsAbleToExtra()
    card::is_race(d, c, RACE_ROCK) && card::is_synchro_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck()
    //     and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c) && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_EXTRA)
        && card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
    {
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
    }
}
