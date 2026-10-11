//! U.A. Turnover Tactics
//! Ｕ．Ａ．ターンオーバー・タクティクス
use super::prelude::*;

pub const ID: u32 = 61613388;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_UA)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UA as u16])
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>=2
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 2
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_UA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_UA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn mzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetSequence()<5 and c:IsControler(tp)
    card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)+g:FilterCount(s.mzfilter,nil,tp)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        if g.is_empty() {
            return false;
        }
        let loc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
        let mz = group::filter_count(d, &g, &|d, c| mzfilter(d, c, a.tp), NO_EX) as i64;
        return loc + mz > 0 && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
fn locfilter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    // return c:IsLocation(LOCATION_DECK) and c:IsControler(sp)
    card::is_location(d, c, LOCATION_DECK) && card::is_controler(d, c, sp)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetMatchingGroup(Card.IsAbleToDeck,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let tg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
    // local ct1=Duel.GetOperatedGroup():FilterCount(s.locfilter,nil,tp)
    let op = duel::get_operated_group(d);
    let mut ct1 = group::filter_count(d, &op, &|d, c| locfilter(d, c, a.tp), NO_EX) as i64;
    // local ct2=Duel.GetOperatedGroup():FilterCount(s.locfilter,nil,1-tp)
    let op = duel::get_operated_group(d);
    let mut ct2 = group::filter_count(d, &op, &|d, c| locfilter(d, c, 1 - a.tp), NO_EX) as i64;
    // if ct1>Duel.GetLocationCount(tp,LOCATION_MZONE) then ct1=Duel.GetLocationCount(tp,LOCATION_MZONE) end
    let lc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    if ct1 > lc {
        ct1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    }
    // if ct2>Duel.GetLocationCount(1-tp,LOCATION_MZONE) then ct2=Duel.GetLocationCount(1-tp,LOCATION_MZONE) end
    let lc2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    if ct2 > lc2 {
        ct2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    }
    // if ct1<=0 then return end
    if ct1 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp)
    let mut g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,1,1,nil)
    let mut sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // g:Remove(Card.IsCode,nil,sg:GetFirst():GetCode())
    let first = group::get_first(d, &sg).nil();
    let codes = card::get_code(d, first);
    group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes), NO_EX);
    // ct1=ct1-1
    ct1 -= 1;
    // while ct1>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) do
    while ct1 > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g1=g:Select(tp,1,1,nil)
        let g1 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,g1:GetFirst():GetCode())
        let first1 = group::get_first(d, &g1).nil();
        let codes1 = card::get_code(d, first1);
        group::remove(d, &mut g, &|d, c| card::is_code(d, c, &codes1), NO_EX);
        // sg:Merge(g1)
        group::add_card(&mut sg, &g1);
        // ct1=ct1-1
        ct1 -= 1;
    }
    // local sc=sg:GetFirst()
    let _ = group::get_first(d, &sg);
    // for sc in aux.Next(sg) do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // Cannot attack this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if ct2>0 and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,0,LOCATION_DECK,1,nil,e,0,1-tp,false,false)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
    if ct2 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, 1 - a.tp, false, false, None, None, None)), a.tp, 0, LOCATION_DECK, 1, NO_EX)
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
        // local sg2=Duel.SelectMatchingCard(1-tp,Card.IsCanBeSpecialSummoned,tp,0,LOCATION_DECK,1,ct2,nil,e,0,1-tp,false,false)
        let sg2 = duel::select_matching_card(
            d,
            1 - a.tp,
            Some(&|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, 1 - a.tp, false, false, None, None, None)),
            a.tp,
            0,
            LOCATION_DECK,
            1,
            ct2 as u16,
            false,
            NO_EX,
        )
        .nil();
        // if #sg2>0 then
        if !sg2.is_empty() {
            // Duel.SpecialSummon(sg2,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg2, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
