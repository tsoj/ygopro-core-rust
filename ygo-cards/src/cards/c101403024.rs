//! Reindsurm Amurusa
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403024;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[101403055], listed_series: &[SET_REINDSURM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // If this card is Ritual Summoned: You can add 2 non-WIND "Reindsurm" monsters from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_ritual_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is sent to the GY, except by battle: You can Ritual Summon 1 Insect Ritual Monster from your hand, by Tributing monsters from your hand or field, and/or shuffling Insect and/or Dragon monsters from your GY into the Deck, whose total Levels equal or exceed its Level
    // local ritual_params={...}
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RELEASE+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_RELEASE + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return not e:GetHandler():IsReason(REASON_BATTLE) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::is_reason(d, h, REASON_BATTLE)
        })),
    );
    // e2:SetTarget(Ritual.Target(ritual_params))
    let target = ritual::target(d, ritual_params());
    effect::set_target(d, e2, Some(target));
    // e2:SetOperation(Ritual.Operation(ritual_params))
    let operation_fn = ritual::operation(d, ritual_params());
    effect::set_operation(d, e2, Some(operation_fn));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// local ritual_params={lvtype=RITPROC_GREATER, filter=..., extratg=..., extraop=..., extrafil=...}
fn ritual_params() -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(|d: &mut Duel, c: CardId| card::is_race(d, c, RACE_INSECT))),
        extratg: Some(cost_target(ritual_extratg)),
        extraop: Some(Arc::new(|d: &mut Duel, mg: &CardSet, _a: &EffArgs, _tc: CardId| ritual_extraop(d, mg))),
        extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| Some(ritual_extrafil(d, a)))),
        ..Default::default()
    }
}
// extratg=function(e,tp,eg,ep,ev,re,r,rp,chk)
fn ritual_extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// extraop=function(mg,e,tp,eg,ep,ev,re,r,rp)
fn ritual_extraop(d: &mut Duel, mg: &CardSet) {
    // local gymg,hfmg=mg:Split(Card.IsLocation,nil,LOCATION_GRAVE)
    let (gymg, hfmg) = group::split(d, mg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #hfmg>0 then Duel.ReleaseRitualMaterial(hfmg) end
    if !hfmg.is_empty() {
        duel::release_ritual_material(d, &hfmg, false);
    }
    // if #gymg>0 then
    if !gymg.is_empty() {
        // Duel.HintSelection(gymg)
        duel::hint_selection(d, gymg.clone(), true);
        // Duel.SendtoDeck(gymg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_RITUAL|REASON_MATERIAL)
        duel::send_to_deck(d, gymg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_RITUAL | REASON_MATERIAL, None);
    }
}
// extrafil=function(e,tp,mg)
fn ritual_extrafil(d: &mut Duel, a: &EffArgs) -> CardSet {
    // local matfilter=not Duel.IsChainSolving() and s.extramatfilter or aux.NecroValleyFilter(s.extramatfilter)
    // return Duel.GetMatchingGroup(matfilter,tp,LOCATION_GRAVE,0,nil)
    if !duel::is_chain_solving(d) {
        duel::get_matching_group(d, Some(&extramatfilter), a.tp, LOCATION_GRAVE, 0, NO_EX)
    } else {
        duel::get_matching_group(d, Some(&aux::necro_valley_filter(extramatfilter)), a.tp, LOCATION_GRAVE, 0, NO_EX)
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttributeExcept(ATTRIBUTE_WIND) and c:IsSetCard(SET_REINDSURM) and c:IsMonster() and c:IsAbleToHand()
    card::is_attribute_except(d, c, ATTRIBUTE_WIND, None, 0, None) && card::is_set_card(d, c, &[SET_REINDSURM as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.extramatfilter(c)
fn extramatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT|RACE_DRAGON) and c:HasLevel() and c:IsAbleToDeck()
    card::is_race(d, c, (RACE_INSECT | RACE_DRAGON) as u64) && card::has_level(d, c) && card::is_able_to_deck(d, c)
}
