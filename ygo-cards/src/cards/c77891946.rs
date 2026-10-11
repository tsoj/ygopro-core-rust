//! Exosister Vadis
//! Scripted by Hel
use super::prelude::*;

pub const ID: u32 = 77891946;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.lfilter1,1,nil,sg)
    let r = group::is_exists(d, sg, &|d, c| lfilter1(d, c, sg), 1, NO_EX);
    (r, false)
}
/// `s.lfilter1(c,sg)`
fn lfilter1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return sg:IsExists(s.lfilter2,1,c,c)
    group::is_exists(d, sg, &|d, x| lfilter2(d, x, c), 1, c.into())
}
/// `s.lfilter2(c,tc)`
fn lfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:ListsCode(tc:GetCode())
    let code = card::get_code(d, tc)[0];
    card::lists_code(d, c, &[code])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local mg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 1 {
            return false;
        }
        return matches!(aux::select_unselect_group(d, &mg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, 0, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local mg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #mg<2 then return end
    if mg.len() < 2 {
        return;
    }
    // local g1=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let g1 = aux::select_unselect_group(d, &mg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #g1>1 then
    if g1.len() > 1 {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // local tc=g1:GetFirst()
        let _tc = group::get_first(d, &g1);
        // for tc in aux.Next(g1) do
        for tc in group::iter(d, &g1) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // g1:KeepAlive()  (nothing: groups are values)
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, e1, CATEGORY_TODECK);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetLabelObject(g1)
        effect::set_label_object(d, e1, Value::Group(g1.clone()));
        // e1:SetCondition(s.tdcon)
        effect::set_condition(d, e1, Some(condition(tdcon)));
        // e1:SetOperation(s.tdop)
        effect::set_operation(d, e1, Some(operation(tdop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetTarget(s.splimit)
        effect::set_target(d, e1, Some(effect_card_fn(splimit)));
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // --Lizard check
        // aux.addTempLizardCheck(c,tp,s.lizfilter)
        aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    }
}
/// `s.tdfilter(c,fid)`
fn tdfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.tdfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| tdfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // local tg=g:Filter(s.tdfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| tdfilter(d, c, fid), NO_EX);
    // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_EXOSISTER) and c:IsLocation(LOCATION_EXTRA)
    !card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsOriginalSetCard(SET_EXOSISTER)
    !card::is_original_set_card(d, c, &[SET_EXOSISTER as u16])
}
