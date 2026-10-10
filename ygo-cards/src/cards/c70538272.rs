//! Filia Regis
use super::prelude::*;

pub const ID: u32 = 70538272;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Dragon Fusion Monster + 1 Level 7 or higher Dragon monster
    // Fusion.AddProcMix(c,true,true,s.matfilter1,s.matfilter2)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter1(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter2(d, c, fc, st, tp))),
        ],
    );
    // Banish 1 card your opponent controls or in their GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 Dragon monster you control to the hand/Extra Deck, and if you do, Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE_START);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter1(c,fc,sumtype,tp)`
fn matfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_FUSION,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp))
}
/// `s.matfilter2(c,fc,sumtype,tp)`
fn matfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsLevelAbove(7)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_level_above(d, c, 7)
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_GRAVE) or Duel.IsExistingMatchingCard(s.rethandfilter,tp,LOCATION_MZONE,0,1,nil))
    card::is_able_to_remove(d, c, None, None, None)
        && (card::is_location(d, c, LOCATION_GRAVE) || duel::is_existing_matching_card(d, Some(&|d, x| rethandfilter(d, x)), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
/// `s.rethandfilter(c)`
fn rethandfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAbleToHand() and c:IsFaceup()
    card::is_race(d, c, RACE_DRAGON) && card::is_able_to_hand(d, c, None) && card::is_faceup(d, c)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.rmfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && rmfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g.clone(), 1, a.tp, 0);
    // if g:GetFirst():IsLocation(LOCATION_ONFIELD) then
    let first = group::get_first(d, &g.nil()).nil();
    if card::is_location(d, first, LOCATION_ONFIELD) {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    }
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsPreviousLocation(LOCATION_ONFIELD) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_previous_location(d, tc, LOCATION_ONFIELD) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.rethandfilter,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rethandfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0 and c:IsRace(RACE_DRAGON) and (c:IsAbleToHand() or c:IsAbleToExtra()) and c:IsFaceup()
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && card::is_race(d, c, RACE_DRAGON)
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_extra(d, c))
        && card::is_faceup(d, c)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND|LOCATION_EXTRA)
    //     and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
            && card::is_location(d, tc, LOCATION_HAND | LOCATION_EXTRA)
            && card::is_relate_to_effect(d, c, a.e)
        {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
