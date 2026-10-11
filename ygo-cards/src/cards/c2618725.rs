//! GMX Partner Selandea
use super::prelude::*;

pub const ID: u32 = 2618725;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; Special Summon 1 "GMX" monster or 1 Dinosaur monster from your hand,
    // also you can only attack directly with "GMX" monsters for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.handsptg)
    effect::set_target(d, e1, Some(cost_target(handsptg)));
    // e1:SetOperation(s.handspop)
    effect::set_operation(d, e1, Some(operation(handspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Special Summoned by a monster effect: You can Special Summon 1 Level 4 or lower "GMX" or Dinosaur
    // monster from your hand, GY, or banishment in Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end)
    effect::set_condition(d, e2, Some(condition(monster_effect_cond)));
    // e2:SetTarget(s.handgybansptg)
    effect::set_target(d, e2, Some(cost_target(handgybansptg)));
    // e2:SetOperation(s.handgybanspop)
    effect::set_operation(d, e2, Some(operation(handgybanspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return re and re:IsMonsterEffect() end`
fn monster_effect_cond(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => effect::is_monster_effect(d, re),
        None => false,
    }
}
/// `s.gmxdinospfilter(c,e,tp,pos)`
fn gmxdinospfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_GMX) or c:IsRace(RACE_DINOSAUR)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_GMX as u16]) || card::is_race(d, c, RACE_DINOSAUR as u64))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.handsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.gmxdinospfilter,tp,LOCATION_HAND,0,1,nil,e,tp,POS_FACEUP) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gmxdinospfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.handspop(e,tp,eg,ep,ev,re,r,rp)`
fn handspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.gmxdinospfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,POS_FACEUP)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| gmxdinospfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX)
            .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // You can only attack directly with "GMX" monsters for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_GMX) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_GMX as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.handgybanspfilter(c,e,tp)`
fn handgybanspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and s.gmxdinospfilter(c,e,tp,POS_FACEUP_DEFENSE) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_level_below(d, c, 4)
        && gmxdinospfilter(d, c, e, tp)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
/// `s.handgybansptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn handgybansptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.handgybanspfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| handgybanspfilter(d, c, a.e, tp)),
                tp,
                LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.handgybanspop(e,tp,eg,ep,ev,re,r,rp)`
fn handgybanspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.handgybanspfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| handgybanspfilter(d, c, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
