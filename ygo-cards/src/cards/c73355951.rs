//! Alpha Summon
use super::prelude::*;

pub const ID: u32 = 73355951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,targ_p)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, targ_p: u8) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsCanBeEffectTarget(e)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,targ_p)
    card::is_faceup(d, c)
        && card::is_monster(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(targ_p), None)
}
/// `s.sprescon(sg,e,tp,mg)`
fn sprescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetControler)==2
    (group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 2, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g0=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_REMOVED,0,nil,e,tp,1-tp)
    let g0 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, 1 - tp)), tp, LOCATION_REMOVED, 0, NO_EX);
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,0,LOCATION_REMOVED,nil,e,tp,tp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tp)), tp, 0, LOCATION_REMOVED, NO_EX);
    // if chk==0 then return #g0>0 and #g1>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0 end
    if a.chk == Some(0) {
        return !g0.is_empty()
            && !g1.is_empty()
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0;
    }
    // local tg=aux.SelectUnselectGroup(g0+g1,e,tp,2,2,s.sprescon,1,tp,HINTMSG_SPSUMMON)
    let mut gu = g0.clone();
    group::add_card(&mut gu, &g1);
    let tg = aux::select_unselect_group(d, &gu, Some(a.e), tp, Some(2), Some(2), Some(&sprescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // e:SetLabelObject((g0&tg):GetFirst())
    let c0 = group::get_first(d, &group::intersection(&g0, &tg));
    effect::set_label_object(d, a.e, match c0 {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    });
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)==0 then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 == 0
    {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c0=e:GetLabelObject()
    let c0 = effect::get_label_object(d, a.e).as_card();
    // if g:IsContains(c0) and c0:IsControler(tp)
    //     and Duel.SpecialSummonStep(c0,0,tp,1-tp,false,false,POS_FACEUP)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(c0) = c0 {
        if group::is_contains(&g, c0)
            && card::is_controler(d, c0, tp)
            && duel::special_summon_step(d, c0, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // local c1=g:RemoveCard(c0):GetFirst()
            group::remove_card(&mut g, &CardSet::from_card(c0));
            let c1 = group::get_first(d, &g);
            // if c1 and c1:IsControler(1-tp) then
            if let Some(c1) = c1 {
                if card::is_controler(d, c1, 1 - tp) {
                    // Duel.SpecialSummonStep(c1,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon_step(d, c1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
