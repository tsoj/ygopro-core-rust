//! Exosister Returnia
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 197042;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not c:IsXyzSummoned() end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_fn));
}
/// `function(c) return not c:IsXyzSummoned() end` (the activity counter filter)
fn counter_fn(d: &mut Duel, a: Args) -> bool {
    !card::is_xyz_summoned(d, a.c(0))
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g>0 and g:FilterCount(aux.FaceupFilter(Card.IsSetCard,SET_EXOSISTER),nil)==#g
    !g.is_empty() && group::filter_count(d, &g, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_EXOSISTER as u16]), NO_EX) == g.len()
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE)
            && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsXyzSummonable()
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_xyz_summonable(d, c, None, None, 0, 0)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)<1 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) < 1 {
        return;
    }
    // local xg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil)
    let xg = duel::get_matching_group(d, Some(&spfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=#xg>0
    let b1 = !xg.is_empty();
    // local b2=#rg>0 and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)>0
    let b2 = !rg.is_empty() && duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) > 0;
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // if not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=xg:Select(tp,1,1,nil)
        let g = group::select(d, &xg, a.tp, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.XyzSummon(tp,g:GetFirst())
            let sc = group::get_first(d, &g).nil();
            duel::xyz_summon(d, a.tp, sc, None, None, 0, 0);
        }
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=rg:Select(tp,1,1,nil)
        let g = group::select(d, &rg, a.tp, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
