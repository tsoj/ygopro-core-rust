//! Evil Eye Awakening
use super::prelude::*;

pub const ID: u32 = 17616743;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EVIL_EYE_SELENE], listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local locations=LOCATION_HAND|LOCATION_GRAVE
    let mut locations = LOCATION_HAND | LOCATION_GRAVE;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_EVIL_EYE_SELENE),tp,LOCATION_STZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_EVIL_EYE_SELENE])), tp, LOCATION_STZONE, 0, 1, NO_EX) {
        // locations=locations|LOCATION_DECK
        locations |= LOCATION_DECK;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,locations,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, locations, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,locations)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, locations as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local locations=LOCATION_HAND|LOCATION_GRAVE
    let mut locations = LOCATION_HAND | LOCATION_GRAVE;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_EVIL_EYE_SELENE),tp,LOCATION_STZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_EVIL_EYE_SELENE])), tp, LOCATION_STZONE, 0, 1, NO_EX) {
        // locations=locations|LOCATION_DECK
        locations |= LOCATION_DECK;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,locations,0,1,1,nil,e,tp)
    let necro = aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&necro), tp, locations, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
