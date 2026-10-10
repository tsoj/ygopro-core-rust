//! Dogmatika Encounter
use super::prelude::*;

pub const ID: u32 = 29354228;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.handspfilter(c,e,tp)`
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_DOGMATIKA) or c:IsCode(CARD_ALBAZ)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) || card::is_code(d, c, &[CARD_ALBAZ]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.thspfilter(c,e,tp,mzone_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mzone_chk: bool) -> bool {
    // return (c:IsSetCard(SET_DOGMATIKA) or c:IsCode(CARD_ALBAZ)) and c:IsMonster() and (c:IsAbleToHand()
    //     or (mzone_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) || card::is_code(d, c, &[CARD_ALBAZ]))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mzone_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mzone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local b1=mzone_chk and Duel.IsExistingMatchingCard(s.handspfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b1 = mzone_chk && duel::is_existing_matching_card(d, Some(&|d, c| handspfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,mzone_chk)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, tp, mzone_chk)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    let op = op.map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Special Summon 1 "Dogmatika" monster or "Fallen of Albaz" from your hand
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.handspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| handspfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Add to your hand or Special Summon 1 "Dogmatika" monster or "Fallen of Albaz" from your GY
        // local mzone_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mzone_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,mzone_chk):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, a.e, tp, mzone_chk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // aux.ToHandOrElse(sc,tp,
        //     function() return mzone_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,4)
        // )
        let e = a.e;
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, _c| mzone_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
            Some(&|d, _g| {
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 4)),
        );
    }
}
