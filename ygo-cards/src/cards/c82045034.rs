//! Thunder Dragons' Hundred Thunders
//! 百雷のサンダー・ドラゴン
use super::prelude::*;

pub const ID: u32 = 82045034;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 Thunder monster from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`: `c:IsRace(RACE_THUNDER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_THUNDER) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c,code,e,tp)`: `c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)`
fn spfilter2(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.splimit(e,c)`: `return not c:IsRace(RACE_THUNDER)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_THUNDER)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // local code=tc:GetCode()
    let tc = tc_opt.nil();
    let code = card::get_code(d, tc)[0];
    // if tc and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetAbsoluteRange(tp,1,0)
        effect::set_absolute_range(d, e2, tp, 1, 0);
        // e2:SetTarget(s.splimit)
        effect::set_target(d, e2, Some(effect_card_fn(splimit)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
        // local ct=Duel.SpecialSummonComplete()
        let ct = duel::special_summon_complete(d);
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // local sg=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_GRAVE,0,nil,code,e,tp)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter2(d, c, code, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        // if ct==0 or ft<=0 or #sg==0 then return end
        if ct == 0 || ft <= 0 || sg.is_empty() {
            return;
        }
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local g=sg:Select(tp,ft,ft,nil)
            let g = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
            // local dg=g:GetFirst()
            let _dg = group::get_first(d, &g);
            // for dg in aux.Next(g) do
            for dg in group::iter(d, &g) {
                // Duel.SpecialSummonStep(dg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, dg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                // local e3=Effect.CreateEffect(e:GetHandler())
                let e3 = effect::create_effect(d, h);
                // e3:SetDescription(3300)
                effect::set_description(d, e3, 3300);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
                effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e3:SetValue(LOCATION_REMOVED)
                effect::set_value(d, e3, LOCATION_REMOVED as i64);
                // e3:SetReset(RESET_EVENT|RESETS_REDIRECT)
                effect::set_reset(d, e3, RESET_EVENT | RESETS_REDIRECT, 1);
                // dg:RegisterEffect(e3,true)
                card::register_effect(d, dg, e3, true);
                // local e4=Effect.CreateEffect(e:GetHandler())
                let e4 = effect::create_effect(d, h);
                // e4:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e4, EFFECT_TYPE_FIELD);
                // e4:SetRange(LOCATION_MZONE)
                effect::set_range(d, e4, LOCATION_MZONE);
                // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e4:SetAbsoluteRange(tp,1,0)
                effect::set_absolute_range(d, e4, tp, 1, 0);
                // e4:SetTarget(s.splimit)
                effect::set_target(d, e4, Some(effect_card_fn(splimit)));
                // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
                // dg:RegisterEffect(e4,true)
                card::register_effect(d, dg, e4, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
