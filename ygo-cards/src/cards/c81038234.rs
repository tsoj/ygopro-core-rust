//! Dream Mirror Recap
use super::prelude::*;

pub const ID: u32 = 81038234;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR],
    listed_series: &[SET_DREAM_MIRROR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your "Dream Mirror" monsters are shuffled into the Deck instead of being sent to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return c:IsSetCard(SET_DREAM_MIRROR) and c:IsReason(REASON_COST) and c:GetReasonEffect():GetHandler()==c end)
    effect::set_target(d, e2, Some(effect_card_fn(redirect_target)));
    // e2:SetValue(LOCATION_DECKSHF)
    effect::set_value(d, e2, LOCATION_DECKSHF as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 "Dream Mirror of Joy" or "Dream Mirror of Terror" face-up in your Field Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END));
    // e3:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e3, Some(condition(main_phase_cond)));
    // e3:SetCost(s.plcost)
    effect::set_cost(d, e3, Some(cost_target(plcost)));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,c) return c:IsSetCard(SET_DREAM_MIRROR) and c:IsReason(REASON_COST) and c:GetReasonEffect():GetHandler()==c end` (e2)
fn redirect_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    if !(card::is_set_card(d, c, &[SET_DREAM_MIRROR as u16]) && card::is_reason(d, c, REASON_COST)) {
        return false;
    }
    let re = card::get_reason_effect(d, c).nil();
    effect::get_handler(d, re).nil() == c
}
/// `function() return Duel.IsMainPhase() end` (e3)
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsCode(CARD_DREAM_MIRROR_JOY,CARD_DREAM_MIRROR_TERROR) and c:IsFaceup() and not c:IsForbidden()
    card::is_field_spell(d, c) && card::is_code(d, c, &[CARD_DREAM_MIRROR_JOY, CARD_DREAM_MIRROR_TERROR]) && card::is_faceup(d, c) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:ListsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::lists_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local plc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not plc then return end
    let Some(plc) = group::get_first(d, &g) else {
        return;
    };
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // if not Duel.MoveToField(plc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if !duel::move_to_field(d, plc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local code=plc:GetCode()
    let code = card::get_code(d, plc)[0];
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp,code)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, code)), a.tp, LOCATION_HAND, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sg = group::get_first(d, &sel).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
