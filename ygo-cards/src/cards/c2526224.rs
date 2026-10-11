//! Fire King High Avatar Kirin
//! --炎王神獣 キリン
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 2526224;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_FIRE_KING],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand during the Main Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Fire King" monster from your hand or GY then destroy 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsReason(REASON_DESTROY) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_reason(d, c, REASON_DESTROY)
    })));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,c,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| desfilter(d, x, tp)),
            tp,
            LOCATION_MZONE | LOCATION_HAND,
            0,
            1,
            c.into(),
        ) && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let tp = a.tp;
    // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,exc,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| desfilter(d, x, tp)),
        tp,
        LOCATION_MZONE | LOCATION_HAND,
        0,
        1,
        1,
        false,
        exc.into(),
    )
    .nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FIRE_KING) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_FIRE_KING as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| spfilter(d, x, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, LOCATION_ONFIELD as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
            let mut dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
            // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // dg=dg:Select(tp,1,1,nil)
                dg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(dg,true)
                duel::hint_selection(d, dg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(dg,REASON_EFFECT)
                duel::destroy(d, dg, REASON_EFFECT, None, None);
            }
        }
    }
}
