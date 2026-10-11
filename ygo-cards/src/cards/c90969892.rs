//! Tribe Drive
use super::prelude::*;

pub const ID: u32 = 90969892;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to hand or Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.thspcon)
    effect::set_condition(d, e1, Some(condition(thspcon)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.getfieldraces(tp)`: returns `races, count`
fn getfieldraces(d: &mut Duel, tp: u8) -> (u64, u32) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local races=0
    let mut races: u64 = 0;
    // for tc in aux.Next(g) do races=races|tc:GetRace() end
    for tc in group::iter(d, &g) {
        races |= card::get_race(d, tc);
    }
    // local pop,count=races,0
    let mut pop = races;
    let mut count: u32 = 0;
    // while pop>0 do if (pop&1)>0 then count=count+1 end pop=pop>>1 end
    while pop > 0 {
        if (pop & 1) > 0 {
            count += 1;
        }
        pop >>= 1;
    }
    // return races,count
    (races, count)
}
/// `s.thspfilter(c,e,tp,ft,races)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32, races: u64) -> bool {
    // return c:IsRace(races) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_race(d, c, races)
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thsprescon(sg)`
fn thsprescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetRace)==#sg
    (group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == sg.len(), false)
}
/// `s.thspcon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local races,count=s.getfieldraces(tp)
    let (races, count) = getfieldraces(d, tp);
    // if count<3 then return false end
    if count < 3 {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_DECK,0,nil,e,tp,ft,races)
    let g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, e, tp, ft, races)), tp, LOCATION_DECK, 0, NO_EX);
    // return aux.SelectUnselectGroup(g,e,tp,3,3,s.thsprescon,0)
    matches!(
        aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&thsprescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local races,count=s.getfieldraces(tp)
    let (races, count) = getfieldraces(d, tp);
    // if count<3 then return end
    if count < 3 {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_DECK,0,nil,e,tp,ft,races)
    let g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, e, tp, ft, races)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.thsprescon,1,tp,HINTMSG_CONFIRM)
    let mut sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(3),
        Some(3),
        Some(&thsprescon),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg~=3 then return end
    if sg.len() != 3 {
        return;
    }
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // local sc=sg:RandomSelect(1-tp,1):GetFirst()
    let picked = group::random_select(d, &sg, 1 - tp, 1);
    // if not sc then return end
    let Some(sc) = group::get_first(d, &picked) else { return };
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // aux.ToHandOrElse(sc,tp,function(sc) return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function(sc) Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end, aux.Stringid(id,1))
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, c| ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| {
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            0
        }),
        Some(aux::stringid(ID, 1)),
    );
    // sg=sg-sc
    group::remove_card(&mut sg, &CardSet::from_card(sc));
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.MoveToDeckBottom(sg,tp)
        duel::move_to_deck_bottom(d, sg.clone());
        // Duel.SortDeckbottom(tp,tp,#sg)
        duel::sort_deckbottom(d, tp, tp, sg.len() as u16);
    }
}
