//! GMX 55th Experiment Report
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 38984832;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Dinosaur Fusion Monster from your Extra Deck using monsters from your hand or field.
    // If your opponent controls a monster, you can also use 1 "GMX" monster in your Deck as material
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=function(c) return c:IsRace(RACE_DINOSAUR) end,
    //     extrafil=s.fextra,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DINOSAUR as u64))),
            extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| fextra(d, e, tp, mg))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase: You can banish this card from your GY; excavate the top cards of your Deck until you
    // excavate a "GMX" card, add that "GMX" card to your hand, also shuffle the rest into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.excavtg)
    effect::set_target(d, e2, Some(cost_target(excavtg)));
    // e2:SetOperation(s.excavop)
    effect::set_operation(d, e2, Some(operation(excavop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck_fn() -> LuaFn {
    lua_fn(|d, a: Args| -> bool {
        // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=1
        let sg = a.g(1);
        group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) <= 1
    })
}
/// `s.deckmatfilter(c)`
fn deckmatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GMX) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_GMX as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 then
    if duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0 {
        // local eg=Duel.GetMatchingGroup(s.deckmatfilter,tp,LOCATION_DECK,0,nil)
        let eg = duel::get_matching_group(d, Some(&deckmatfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #eg>0 then return eg,s.fcheck end
        if !eg.is_empty() {
            return (Some(eg), Some(fcheck_fn()));
        }
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GMX) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GMX as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.excavtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `g:GetMaxGroup(Card.GetSequence):GetFirst()`
fn max_seq_card(d: &mut Duel, g: &CardSet) -> CardId {
    let max = group::get_max_group(d, g, &|d, c| card::get_sequence(d, c) as i64).nil().0;
    group::get_first(d, &max).nil()
}
/// `s.excavop(e,tp,eg,ep,ev,re,r,rp)`
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if deck_count>0 then
    if deck_count > 0 {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 then
        if g.is_empty() {
            // Duel.ConfirmDecktop(tp,deck_count)
            duel::confirm_decktop(d, tp, deck_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,deck_count)
            let excav_g = duel::get_decktop_group(d, tp, deck_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,deck_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, deck_count as u32);
        } else {
            // local sc=g:GetMaxGroup(Card.GetSequence):GetFirst()
            let sc = max_seq_card(d, &g);
            // local sc_seq=sc:GetSequence()
            let sc_seq = card::get_sequence(d, sc) as i32;
            // local excav_count=deck_count-sc_seq
            let excav_count = deck_count - sc_seq;
            // Duel.ConfirmDecktop(tp,excav_count)
            duel::confirm_decktop(d, tp, excav_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,excav_count)
            let excav_g = duel::get_decktop_group(d, tp, excav_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,excav_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, excav_count as u32);
            // if Duel.SendtoHand(sc,nil,REASON_EFFECT) then
            //     Duel.ConfirmCards(1-tp,sc)
            //     Duel.ShuffleHand(tp)
            // end
            // (a Lua number is always true, so the body always runs)
            duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, sc);
            duel::shuffle_hand(d, tp);
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
}
