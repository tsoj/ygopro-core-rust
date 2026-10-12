//! The Immortal Sun God
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403051;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_RA],
    listed_series: &[SET_SUN_GOD, SET_THE_WINGED_DRAGON_OF_RA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "The Winged Dragon of Ra" monster from your GY or banishment, ignoring its Summoning conditions, then if you Special Summoned "The Winged Dragon of Ra" by this effect, you can send it to the GY, and if you do, Special Summon 1 "The Winged Dragon of Ra" monster from your Extra Deck. During the End Phase, send the monsters Special Summoned by this effect to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if this card is in your GY: You can discard 1 "Sun God" card; add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.Discard(function(c) return c:IsSetCard(SET_SUN_GOD) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::discard(
            Some(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| card::is_set_card(d, c, &[SET_SUN_GOD as u16]))),
            false,
            None,
            None,
            None,
        )),
    );
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.gybanspfilter(c,e,tp)
fn gybanspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_THE_WINGED_DRAGON_OF_RA) and c:IsMonster() and c:IsFaceup()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_THE_WINGED_DRAGON_OF_RA as u16])
        && card::is_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.gybanspfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| gybanspfilter(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.extraspfilter(c,e,tp,exc)
fn extraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, exc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_THE_WINGED_DRAGON_OF_RA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,exc,c)>0
    card::is_set_card(d, c, &[SET_THE_WINGED_DRAGON_OF_RA as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(
            d,
            tp,
            Some(tp),
            match exc {
                Some(x) => x.into(),
                None => NO_EX,
            },
            duel::ExtraCard::Card(c),
            None,
        )
        .0 > 0
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.gybanspfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| gybanspfilter(d, c, e, tp))),
        tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let mut sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,true,false,POS_FACEUP) and sc:IsCode(CARD_RA) then
    if let Some(s) = sc {
        if duel::special_summon_step(d, s, 0, tp, tp, true, false, POS_FACEUP as u8, None) && card::is_code(d, s, &[CARD_RA]) {
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // if sc:IsAbleToGrave() and Duel.IsExistingMatchingCard(s.extraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sc)
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if card::is_able_to_grave(d, s, None)
                && duel::is_existing_matching_card(d, Some(&|d, c| extraspfilter(d, c, e, tp, Some(s))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
                && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // if Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) then
                if duel::send_to_grave(d, CardSet::from_card(s), REASON_EFFECT, None, None) > 0 && card::is_location(d, s, LOCATION_GRAVE) {
                    // sc=nil
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                    // sc=Duel.SelectMatchingCard(tp,s.extraspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
                    let g2 = duel::select_matching_card(
                        d,
                        tp,
                        Some(&|d, c| extraspfilter(d, c, e, tp, None)),
                        tp,
                        LOCATION_EXTRA,
                        0,
                        1,
                        1,
                        false,
                        NO_EX,
                    )
                    .nil();
                    sc = group::get_first(d, &g2);
                    // if sc then
                    if let Some(s2) = sc {
                        // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
                        duel::special_summon_step(d, s2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    }
                }
            }
        }
    }
    // if sc then
    if let Some(s) = sc {
        // During the End Phase, send the monsters Special Summoned by this effect to the GY
        // aux.DelayedOperation(sc,PHASE_END,id,e,tp,function(ag) Duel.SendtoGrave(ag,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,3))
        aux::delayed_operation(
            d,
            s,
            PHASE_END,
            ID,
            e,
            tp,
            Some(Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                duel::send_to_grave(d, ag.clone(), REASON_EFFECT, None, None);
                true
            })),
            None,
            Some(0),
            Some(0),
            Some(aux::stringid(ID, 3)),
            None,
        );
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
