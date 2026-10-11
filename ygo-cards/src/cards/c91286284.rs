//! Gold Pride - That Came Out of Nowhere!
use super::prelude::*;

pub const ID: u32 = 91286284;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOLD_PRIDE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // return c:IsSetCard(SET_GOLD_PRIDE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOLD_PRIDE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),1-tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,0,1-tp,false,false)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, 1 - tp, false, false, None, None, None))),
            1 - tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        // local tc=Duel.SelectMatchingCard(1-tp,aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),1-tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,0,1-tp,false,false):GetFirst()
        let g2 = duel::select_matching_card(
            d,
            1 - tp,
            Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, 1 - tp, false, false, None, None, None))),
            1 - tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if not tc then return end
        let Some(tc) = group::get_first(d, &g2) else {
            return;
        };
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummonStep(tc,0,1-tp,1-tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None) {
            // tc:NegateEffects(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, tc, h, None, false, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
