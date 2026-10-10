//! Revolution Synchron
use super::prelude::*;

pub const ID: u32 = 97682931;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can be used as material from the hand for "Power Tool" Synchro Monster or Level 7/8 Dragon Monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_SYNCHRO_MAT_FROM_HAND)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MAT_FROM_HAND);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetValue(s.synval)
    effect::set_value_fn(d, e1, lua_fn(synval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send the top card from your Deck to the GY and Special Summon itself from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DECKDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DECKDES + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// (The Lua script sets `s.lsited_series`, a misspelled name the core never reads: no listed series.)
/// `s.synval(e,mc,sc)`: this effect, this card and the monster to be summoned
fn synval(d: &mut Duel, a: Args) -> bool {
    // return sc:IsSetCard(SET_POWER_TOOL) or (sc:IsRace(RACE_DRAGON) and sc:IsLevel(7,8))
    let sc = a.c(2);
    card::is_set_card(d, sc, &[SET_POWER_TOOL as u16]) || (card::is_race(d, sc, RACE_DRAGON) && card::is_level(d, sc, &[7, 8]))
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsLevelAbove(7)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_level_above(d, c, 7)
}
/// `s.spcon(e,tp,...)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.DiscardDeck(tp,1,REASON_EFFECT)>0 and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_GRAVE) then
    if duel::discard_deck(d, a.tp, 1, REASON_EFFECT) > 0 {
        let og = duel::get_operated_group(d);
        let first = group::get_first(d, &og).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
            if card::is_relate_to_effect(d, c, a.e) && duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
                // --Level becomes 1
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CHANGE_LEVEL)
                effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // c:RegisterEffect(e1,true)
                card::register_effect(d, c, e1, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
