//! Exceed the Pendulum
use super::prelude::*;

pub const ID: u32 = 92812851;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,nil,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, Some(group_check(matcheck)), None, None);
    // Gains 100 ATK for each Pendulum Monster Card you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 face-up Pendulum Monster from your Extra Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Pendulum Monster from the hand or GY in Defense Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsMainPhase() and Duel.GetFieldCard(tp,LOCATION_PZONE,0) and Duel.GetFieldCard(tp,LOCATION_PZONE,1) end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_PENDULUM,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.pendfilter(c)`
fn pendfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_PENDULUM) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_PENDULUM) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.pendfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)*100
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&pendfilter), p, LOCATION_ONFIELD, 0, NO_EX) as i64 * 100
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end` (the condition of e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `function(e,tp) return Duel.IsMainPhase() and Duel.GetFieldCard(tp,LOCATION_PZONE,0) and Duel.GetFieldCard(tp,LOCATION_PZONE,1) end` (the condition of e3)
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
        && duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).is_some()
        && duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).is_some()
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.spfilter(c,e,tp,lsc,rsc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lsc: u32, rsc: u32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsType(TYPE_PENDULUM) and c:HasLevel() and lv>lsc and lv<rsc
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_type(d, c, TYPE_PENDULUM)
        && card::has_level(d, c)
        && lv > lsc
        && lv < rsc
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lsc=Duel.GetFieldCard(tp,LOCATION_PZONE,0):GetLeftScale()
    let lc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).nil();
    let mut lsc = card::get_left_scale(d, lc);
    // local rsc=Duel.GetFieldCard(tp,LOCATION_PZONE,1):GetRightScale()
    let rc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).nil();
    let mut rsc = card::get_right_scale(d, rc);
    // if lsc>rsc then lsc,rsc=rsc,lsc end
    if lsc > rsc {
        std::mem::swap(&mut lsc, &mut rsc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil,e,tp,lsc,rsc) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, lsc, rsc)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lc=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    let lc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 0);
    // local rc=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    let rc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 1);
    // if not (lc and rc) then return end
    let (Some(lc), Some(rc)) = (lc, rc) else {
        return;
    };
    // local lsc=lc:GetLeftScale()
    let mut lsc = card::get_left_scale(d, lc);
    // local rsc=rc:GetRightScale()
    let mut rsc = card::get_right_scale(d, rc);
    // if lsc>rsc then lsc,rsc=rsc,lsc end
    if lsc > rsc {
        std::mem::swap(&mut lsc, &mut rsc);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil,e,tp,lsc,rsc)
    let (e, tp) = (a.e, a.tp);
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, lsc, rsc))), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
