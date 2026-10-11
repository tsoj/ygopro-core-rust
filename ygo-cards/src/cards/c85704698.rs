//! Re-dyce-cle
use super::prelude::*;

pub const ID: u32 = 85704698;

// s.roll_dice=true
// s.listed_series={SET_SPEEDROID}
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DICE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DICE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Synchro Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER));
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.syntg)
    effect::set_target(d, e2, Some(cost_target(syntg)));
    // e2:SetOperation(s.synop)
    effect::set_operation(d, e2, Some(operation(synop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SPEEDROID as u16])
        && card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    if !duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e2,true)
    card::register_effect(d, tc, e2, true);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local lv=Duel.TossDice(tp,1)
    let lv = duel::toss_dice(d, tp, 1, 0)[0];
    // local e3=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e3 = effect::create_effect(d, h);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
    // e3:SetValue(lv)
    effect::set_value(d, e3, lv as i64);
    // e3:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPEEDROID) and c:IsType(TYPE_TUNER)
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_type(d, c, TYPE_TUNER)
}
/// `s.cfilter(c,syn)`
fn cfilter(d: &mut Duel, c: CardId, syn: CardId) -> bool {
    // return syn:IsSynchroSummonable(c)
    card::is_synchro_summonable(d, syn, Some(CardSet::from_card(c)), None, 0, 0)
}
/// `s.spfilter(c,mg)`
fn spfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and mg:IsExists(s.cfilter,1,nil,c)
    card::is_attribute(d, c, ATTRIBUTE_WIND) && group::is_exists(d, mg, &|d, x| cfilter(d, x, c), 1, NO_EX)
}
/// `s.syntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn syntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local mg=Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&mfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,mg)
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.synop(e,tp,eg,ep,ev,re,r,rp)`
fn synop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local mg=Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&mfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
        // local tg=mg:FilterSelect(tp,s.cfilter,1,1,nil,sg:GetFirst())
        let sg_first = group::get_first(d, &sg).nil();
        let tg = group::filter_select(d, &mg, tp, &|d, x| cfilter(d, x, sg_first), 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),tg:GetFirst())
        let tc = group::get_first(d, &tg);
        duel::synchro_summon(d, tp, sg_first, tc.map(CardSet::from_card), None, 0, 0);
    }
}
