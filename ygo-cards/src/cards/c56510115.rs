//! Solfachord Symphony
use super::prelude::*;

pub const ID: u32 = 56510115;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD, SET_GRANSOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY+CATEGORY_DRAW+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.solfilter(c)`
fn solfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM)
}
/// The number of different codes among the Pendulum Monsters in the Extra Deck (`GetClassCount(Card.GetCode)`).
fn extra_code_classes(d: &mut Duel, tp: u8) -> usize {
    let g = duel::get_matching_group(d, Some(&solfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64)
}
/// `s.acttg(e,tp,...,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SOLFACHORD),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.GetMatchingGroup(s.solfilter,tp,LOCATION_EXTRA,0,nil):GetClassCount(Card.GetCode)>2 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SOLFACHORD as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && extra_code_classes(d, tp) > 2;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GRANSOLFACHORD) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GRANSOLFACHORD as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.actop(e,tp,...)`
fn actop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ct=Duel.GetMatchingGroup(s.solfilter,tp,LOCATION_EXTRA,0,nil):GetClassCount(Card.GetCode)
    let ct = extra_code_classes(d, tp);
    // if ct<3 then return end
    if ct < 3 {
        return;
    }
    // local breakeff=false
    let mut breakeff = false;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // "Solfachord" Pendulum Monsters gain ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetValue(function(e,c) return c:GetScale()*300 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_scale(d, c) as i64 * 300));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // if ct<5 then return end
    if ct < 5 {
        return;
    }
    // Destroy 1 card
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local pg=g:Select(tp,1,1,nil)
        let pg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #pg>0 then
        if !pg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Destroy(pg,REASON_EFFECT)>0 and Duel.IsExistingMatchingCard(Card.IsOddScale,tp,LOCATION_PZONE,0,1,nil) then
            if duel::destroy(d, pg, REASON_EFFECT, None, None) > 0
                && duel::is_existing_matching_card(d, Some(&|d, x| card::is_odd_scale(d, x)), tp, LOCATION_PZONE, 0, 1, NO_EX)
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Draw(tp,1,REASON_EFFECT)
                duel::draw(d, tp, 1, REASON_EFFECT);
            }
            // breakeff=true
            breakeff = true;
        }
    }
    // if ct<7 then return end
    if ct < 7 {
        return;
    }
    // Special Summon 1 "GranSolfachord" monster
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local ssg=sg:Select(tp,1,1,nil)
        let ssg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
        // if #ssg==0 then return end
        if ssg.is_empty() {
            return;
        }
        // if breakeff then Duel.BreakEffect() end
        if breakeff {
            duel::break_effect(d);
        }
        // Duel.SpecialSummon(ssg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, ssg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM)
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM)
}
