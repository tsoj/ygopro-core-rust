//! Alviss of the Nordic Alfar
//! 極星霊アルヴィース
use super::prelude::*;

pub const ID: u32 = 27024795;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC, SET_AESIR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_REMOVE)
    effect::set_code(d, e1, EVENT_REMOVE);
    // e1:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e1, LOCATION_REMOVED);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon when an "Aesir" leaves the field
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
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (for eg:IsContains)
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    // return #eg==1 and eg:IsContains(e:GetHandler()) and r&REASON_EFFECT>0 and re and re:IsActiveType(TYPE_LINK) and re:GetHandler():IsSetCard(SET_NORDIC)
    if eg.len() != 1 || !group::is_contains(&eg, h) || (a.r & REASON_EFFECT) == 0 {
        return false;
    }
    let Some(re) = a.re else { return false };
    if !effect::is_active_type(d, re, TYPE_LINK) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_set_card(d, rh, &[SET_NORDIC as u16])
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NORDIC) and c:IsMonster() and (c:IsFaceup() or c:IsLocation(LOCATION_DECK)) and c:IsLevelAbove(1)
    card::is_set_card(d, c, &[SET_NORDIC as u16])
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
        && card::is_level_above(d, c, 1)
}
/// `s.spcheck(sg,e,tp,mg)`
fn spcheck(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return #sg==3 and sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)==2 and sg:GetSum(Card.GetLevel)==10
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg)
    let e = e.nil();
    let ok = sg.len() == 3
        && group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) == 2
        && group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) == 10
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    let _ = mg;
    (ok, false)
}
/// `s.spfilter(c,e,tp,mg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mg: Option<&CardSet>) -> bool {
    // return c:IsSetCard(SET_AESIR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (not mg or Duel.GetLocationCountFromEx(tp,tp,mg,c)>0)
    if !(card::is_set_card(d, c, &[SET_AESIR as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    match mg {
        None => true,
        Some(mg) => duel::get_location_count_from_ex(d, tp, Some(tp), mg.into(), duel::ExtraCard::Card(c), None).0 > 0,
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_DECK|LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&matfilter), a.tp, LOCATION_DECK | LOCATION_MZONE, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&spcheck), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,3,tp,LOCATION_DECK|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 3, a.tp, (LOCATION_DECK | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_DECK|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&matfilter), a.tp, LOCATION_DECK | LOCATION_MZONE, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,1,tp,HINTMSG_TOGRAVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&spcheck), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #rg==3 and Duel.SendtoGrave(rg,REASON_EFFECT)==3 then
    if rg.len() == 3 && duel::send_to_grave(d, rg, REASON_EFFECT, None, None) == 3 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let (e, tp) = (a.e, a.tp);
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_AESIR) and c:IsSetCard(SET_AESIR) and c:IsPreviousControler(tp)
    card::is_previous_set_card(d, c, &[SET_AESIR as u16]) && card::is_set_card(d, c, &[SET_AESIR as u16]) && card::is_previous_controler(d, c, tp)
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and eg:IsExists(s.cfilter,1,nil,tp)
    if a.rp != 1 - a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.spfilter2(c,e,tp,rp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // return c:IsSetCard(SET_AESIR) and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,c:GetCode())
    if !card::is_set_card(d, c, &[SET_AESIR as u16]) {
        return false;
    }
    if duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let codes = card::get_code(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, cc| card::is_code(d, cc, &codes)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp) end
    if a.chk == Some(0) {
        let (e, tp, rp) = (a.e, a.tp, a.rp);
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp, rp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
