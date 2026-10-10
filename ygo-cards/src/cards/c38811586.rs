//! Albion the Sanctifire Dragon
use super::prelude::*;

pub const ID: u32 = 38811586;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(CARD_ALBAZ), fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp)))],
    );
    // Cannot be used as Fusion Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted by the opponent's effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 2 monsters from the GYs
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e3:SetTarget(s.sptg1)
    effect::set_target(d, e3, Some(cost_target(sptg1)));
    // e3:SetOperation(s.spop1)
    effect::set_operation(d, e3, Some(operation(spop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon itself from the GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RELEASE)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON + CATEGORY_RELEASE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.sptg2)
    effect::set_target(d, e4, Some(cost_target(sptg2)));
    // e4:SetOperation(s.spop2)
    effect::set_operation(d, e4, Some(operation(spop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.matfilter(c,scard,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, scard: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT,scard,sumtype,tp) and c:IsRace(RACE_SPELLCASTER,scard,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(scard), sumtype, Some(tp))
        && card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(scard), sumtype, Some(tp))
}
// function s.spfilter(c,e,tp,targetp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, targetp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,targetp) and c:IsCanBeEffectTarget(e)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(targetp), None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,tp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp,1-tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, 1 - a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return ((#g1>1 and #g2>1) or (#(g1&g2)~=#g1 and #(g1&g2)~=#g2))
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        let inter = group::intersection(&g1, &g2);
        return ((g1.len() > 1 && g2.len() > 1) || (inter.len() != g1.len() && inter.len() != g2.len()))
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // local tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,function(sg) return #(sg&g1)>0 and #(sg&g2)>0 end,1,tp,HINTMSG_SPSUMMON)
    let union = group::union(&g1, &g2);
    let rescon = |_d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (!group::intersection(sg, &g1).is_empty() && !group::intersection(sg, &g2).is_empty(), false)
    };
    let tg = aux::select_unselect_group(
        d,
        &union,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, 0, 0);
    true
}
// function s.spownfilter(c,e,tp,tg)
fn spownfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and (tg-c):GetFirst():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let rest = group::difference(tg, &CardSet::from_card(c));
    let other = group::get_first(d, &rest).nil();
    card::is_can_be_special_summoned(d, other, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
// function s.spop1(e,tp,eg,ep,ev,re,r,rp)
fn spop1(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #tg<2 or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 or Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)<1
    //   or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if tg.len() < 2
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1
        || duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 < 1
        || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=tg:FilterSelect(tp,s.spownfilter,1,1,nil,e,tp,tg):GetFirst()
    let sel = group::filter_select(d, &tg, a.tp, &|d, c| spownfilter(d, c, a.e, a.tp, &tg), 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonStep((tg-sc):GetFirst(),0,tp,1-tp,false,false,POS_FACEUP)
    let rest = group::difference(&tg, &CardSet::from_card(sc));
    let other = group::get_first(d, &rest).nil();
    duel::special_summon_step(d, other, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.spcfilter(c)
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReleasableByEffect() and (c:IsSequence(2) or c:IsInExtraMZone())
    card::is_releasable_by_effect(d, c, None, None) && (card::is_sequence(d, c, &[2]) || card::is_in_extra_m_zone(d, c, None))
}
// function s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&spcfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g==4 and Duel.GetMZoneCount(tp,g)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return g.len() == 4
            && duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop2(e,tp,eg,ep,ev,re,r,rp)
fn spop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&spcfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==4 and Duel.Release(g,REASON_EFFECT)==4 and c:IsRelateToEffect(e) then
    if g.len() == 4 && duel::release(d, g, REASON_EFFECT, None) == 4 && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
