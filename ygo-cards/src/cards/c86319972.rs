//! Gamma Summon
//! Scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 86319972;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALEISTER, SET_INVOKED],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 "Aleister" monster or 1 Fusion Monster from your GY; Special Summon 1 "Invoked" monster from your Extra Deck,
    // ignoring its Summoning conditions, but banish it during the End Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.extraspcost)
    effect::set_cost(d, e1, Some(cost_target(extraspcost)));
    // e1:SetTarget(s.extrasptg)
    effect::set_target(d, e1, Some(cost_target(extrasptg)));
    // e1:SetOperation(s.extraspop)
    effect::set_operation(d, e1, Some(operation(extraspop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a monster(s) is banished face-up, while this card is in your GY: You can banish this card, then target 1 of your
    // banished "Invoked" monsters; Special Summon it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    // (EFFECT is not a defined constant: the Lua passes nil, which the core reads as 0)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.banspcon)
    effect::set_condition(d, e2, Some(condition(banspcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.bansptg)
    effect::set_target(d, e2, Some(cost_target(bansptg)));
    // e2:SetOperation(s.banspop)
    effect::set_operation(d, e2, Some(operation(banspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.extraspcostfilter(c,e,tp)`
fn extraspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_ALEISTER) or c:IsFusionMonster()) and c:IsAbleToRemoveAsCost() and c:IsMonster()
    //     and aux.SpElimFilter(c,true)
    //     and Duel.IsExistingMatchingCard(s.extraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    (card::is_set_card(d, c, &[SET_ALEISTER as u16]) || card::is_fusion_monster(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::is_monster(d, c)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_matching_card(d, Some(&|d, x| extraspfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.extraspfilter(c,e,tp,rc)`
fn extraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_INVOKED) and Duel.GetLocationCountFromEx(tp,tp,rc,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_INVOKED as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), rc.map_or(NO_EX, |rc| rc.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.extraspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extraspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.extraspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| extraspcostfilter(d, c, a.e, tp)),
            tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.extraspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| extraspcostfilter(d, c, a.e, tp)),
        tp,
        LOCATION_MZONE | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.Remove(g,nil,REASON_COST)
    duel::remove(d, g, 0, REASON_COST, None, None);
    true
}
/// `s.extrasptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrasptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-100
        let cost_chk = effect::get_label(d, a.e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk or Duel.IsExistingMatchingCard(s.extraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
        return cost_chk
            || duel::is_existing_matching_card(d, Some(&|d, c| extraspfilter(d, c, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_MZONE as i32);
    true
}
/// `function(ag) Duel.Remove(ag,POS_FACEUP,REASON_EFFECT) end`
fn banish_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::remove(d, ag.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    true
}
/// `s.extraspop(e,tp,eg,ep,ev,re,r,rp)`
fn extraspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.extraspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| extraspfilter(d, c, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX)
        .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) > 0 {
            // Banish it during the End Phase
            // aux.DelayedOperation(sc,PHASE_END,id,e,tp,function(ag) Duel.Remove(ag,POS_FACEUP,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,2))
            aux::delayed_operation(
                d,
                sc,
                PHASE_END,
                ID,
                a.e,
                tp,
                Some(Arc::new(banish_oper)),
                None,
                Some(0),
                Some(0),
                Some(aux::stringid(ID, 2)),
                None,
            );
        }
    }
}
/// `s.banspconfilter(c)`
fn banspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsFaceup() and c:IsLocation(LOCATION_REMOVED) and not c:IsPreviousLocation(LOCATION_SZONE)
    card::is_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_REMOVED)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
}
/// `s.banspcon(e,tp,eg,ep,ev,re,r,rp)`
fn banspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.banspconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &banspconfilter, 1, NO_EX)
}
/// `s.banspfilter(c,e,tp)`
fn banspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_INVOKED) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_INVOKED as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.bansptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn bansptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.banspfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED)
            && card::is_controler(d, chkc, tp)
            && banspfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.banspfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| banspfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.banspfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| banspfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.banspop(e,tp,eg,ep,ev,re,r,rp)`
fn banspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
