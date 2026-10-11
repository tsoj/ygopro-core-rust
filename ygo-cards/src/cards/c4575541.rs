//! Here There Be Dragons
use super::prelude::*;

pub const ID: u32 = 4575541;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGON_RULER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon 1 "Dragon Ruler" monster from your hand, GY, or banishment then you can Xyz Summon 1 "Dragon Ruler" monster
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
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Return any number of your banished "Dragon Ruler" monsters with different Attributes to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.xyzfilter(c,mg)
fn xyzfilter(d: &mut Duel, c: CardId, mg: &Group) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsXyzSummonable(nil,mg)
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16]) && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local mg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DRAGON_RULER),tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])),
            a.tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        // local g=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg)
        let g = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg)), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel).nil();
            // Duel.XyzSummon(tp,sc,nil,mg)
            duel::xyz_summon(d, a.tp, sc, None, Some(mg), 0, 0);
        }
    }
}
// function s.tgfilter(c,e)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsFaceup() and c:IsMonster() and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])
        && card::is_faceup(d, c)
        && card::is_monster(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_REMOVED,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,#g,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_RTOGRAVE)
    let dp = aux::dpcheck(|d, c| card::get_attribute(d, c) as i64);
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(g.len()), Some(&dp), 1, a.tp, Some(HINTMSG_RTOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg), n, a.tp, 0);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, tg, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
