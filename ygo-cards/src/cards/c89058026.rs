//! EN Wave
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 89058026;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    listed_series: &[SET_ELEMENTAL_HERO, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon from Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.dspcon)
    effect::set_condition(d, e1, Some(condition(dspcon)));
    // e1:SetTarget(s.dsptg)
    effect::set_target(d, e1, Some(cost_target(dsptg)));
    // e1:SetOperation(s.dspop)
    effect::set_operation(d, e1, Some(operation(dspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.gspcon)
    effect::set_condition(d, e2, Some(condition(gspcon)));
    // e2:SetTarget(s.gsptg)
    effect::set_target(d, e2, Some(cost_target(gsptg)));
    // e2:SetOperation(s.gspop)
    effect::set_operation(d, e2, Some(operation(gspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dspconfilter(c,tp)`
fn dspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsPreviousControler(tp) and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    //     and c:IsFaceup() and (c:IsMonster() or c:IsPreviousLocation(LOCATION_MZONE))
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_faceup(d, c)
        && (card::is_monster(d, c) || card::is_previous_location(d, c, LOCATION_MZONE))
}
/// `s.dspcon(e,tp,eg,ep,ev,re,r,rp)`
fn dspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return (r&REASON_FUSION)==REASON_FUSION and eg:IsExists(s.dspconfilter,1,nil,tp)
    (a.r & REASON_FUSION) == REASON_FUSION
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| dspconfilter(d, c, tp), 1, NO_EX)
}
/// `s.dspfilter(c,e,tp)`
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_NEO_SPACIAN) or c:IsCode(CARD_NEOS)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) || card::is_code(d, c, &[CARD_NEOS]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), None)
}
/// `s.dsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.dspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| dspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.dspop(e,tp,eg,ep,ev,re,r,rp)`
fn dspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| dspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.gspconfilter(c,tp)`
fn gspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return ((c:IsMonster() and c:IsSetCard(SET_NEO_SPACIAN)) or c:IsCode(CARD_NEOS))
    //     and c:IsPreviousControler(tp) and c:IsControler(tp) and c:IsPreviousLocation(LOCATION_MZONE|LOCATION_GRAVE)
    ((card::is_monster(d, c) && card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16])) || card::is_code(d, c, &[CARD_NEOS]))
        && card::is_previous_controler(d, c, tp)
        && card::is_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE | LOCATION_GRAVE)
}
/// `s.gspcon(e,tp,eg,ep,ev,re,r,rp)`
fn gspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return eg:IsExists(s.gspconfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| gspconfilter(d, c, tp), 1, NO_EX)
}
/// `s.gspfilter(c,e,tp)`
fn gspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), None)
}
/// `s.gsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.gspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gspop(e,tp,eg,ep,ev,re,r,rp)`
fn gspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| gspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
