//! Crossrose Dragon
use super::prelude::*;

pub const ID: u32 = 72218246;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROSE, SET_ROSE_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,nil,s.spcheck)
    link::add_procedure(d, c, None, 2, None, Some(group_check(spcheck)), None, None);
    // synchro summon
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon2)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    effect::set_operation(d, e2, Some(operation(spop2)));
    card::register_effect(d, c, e2, false);
}
/// `s.spcheck(g,lc,sumtype,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetRace,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, x| card::get_race_ctx(d, x, Some(lc), sumtype, Some(tp)))
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT)
    card::is_race(d, c, RACE_PLANT)
}
/// `s.spcon(e,...)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.exkfilter(c,sg,tp,oc)`
fn exkfilter(d: &mut Duel, c: CardId, sg: &CardSet, tp: u8, oc: CardId) -> bool {
    // local sg = sg + oc
    let mut sg = sg.clone();
    group::add_card(&mut sg, &CardSet::from_card(oc));
    // return Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.excheck(sg,tp,exg,ssg,c)`
fn excheck(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet, ssg: &CardSet, c: CardId) -> bool {
    // return ssg:IsExists(s.exkfilter,1,nil,sg,tp,c)
    group::is_exists(d, ssg, &|d, x| exkfilter(d, x, sg, tp, c), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, chk: bool) -> bool {
    // return (c:IsSetCard(SET_ROSE) or c:IsRace(RACE_PLANT)) and c:IsType(TYPE_SYNCHRO) and (not chk or Duel.GetLocationCountFromEx(tp,tp,nil,c)>0)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    (card::is_set_card(d, c, &[SET_ROSE as u16]) || card::is_race(d, c, RACE_PLANT))
        && card::is_type(d, c, TYPE_SYNCHRO)
        && (!chk || duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.spcost(e,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, false)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,s.mzfilter,1,false,s.excheck,c,mg,c)
        return duel::check_release_group_cost(
            d,
            a.tp,
            Some(&mzfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| (excheck(d, sg, tp, exg, &mg, c), false)),
            c.into(),
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.mzfilter,1,1,false,s.excheck,c,mg,c)
    let mut g = duel::select_release_group_cost(
        d,
        a.tp,
        Some(&mzfilter),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| (excheck(d, sg, tp, exg, &mg, c), false)),
        c.into(),
    );
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,true):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, true)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetPreviousControler()==tp and c:IsReason(REASON_EFFECT) and c:GetPreviousTypeOnField()&TYPE_MONSTER~=0
    card::get_previous_controler(d, c) == tp && card::is_reason(d, c, REASON_EFFECT) && card::get_previous_type_on_field(d, c) & TYPE_MONSTER != 0
}
/// `s.spcon2(e,...)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.cfilter,1,nil,tp)
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROSE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ROSE_DRAGON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,...,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_GRAVE,0,1,e:GetHandler(),e,tp)
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop2(e,...)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter2(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
