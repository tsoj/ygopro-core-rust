//! HSR/CW Rider (Hi-Speedroid Clear Wing Rider)
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 86154370;

pub const INFO: ScriptInfo = ScriptInfo { synchro_nt_required: Some(1), roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WIND),1,1,Synchro.NonTunerEx(s.matfilter),1,1)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WIND, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(matfilter)),
        1,
        1,
        None,
        None,
        None,
        None,
        None,
    );
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,val,scard,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND,scard,sumtype,tp) and c:IsType(TYPE_SYNCHRO,scard,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_WIND, sc, st, Some(tp)) && card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp))
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToDeck()
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local d=Duel.TossDice(tp,1)
    let dv = duel::toss_dice(d, tp, 1, 0)[0];
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE,0,1,d,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| tdfilter(d, c))), tp, LOCATION_GRAVE, 0, 1, dv as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local dc=Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        let dc = duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // if not (dc>0 and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2))) then return end
        if !(dc > 0 && duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,dc,nil)
        let dg = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, dc as u16, false, NO_EX).nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // local oc=Duel.Destroy(dg,REASON_EFFECT)
            let oc = duel::destroy(d, dg, REASON_EFFECT, None, None);
            // if oc>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
            if oc > 0 && card::is_relate_to_effect(d, c.nil(), a.e) && card::is_faceup(d, c.nil()) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c.nil());
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(oc*500)
                effect::set_value(d, e1, (oc * 500) as i64);
                // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c.nil(), e1, false);
            }
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return e:GetHandler():IsSynchroSummoned() and Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase()
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && duel::is_turn_player(d, 1 - tp) && duel::is_main_phase(d, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(7) and c:IsType(TYPE_SYNCHRO) and c:IsAttribute(ATTRIBUTE_WIND)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,e:GetHandler(),c)>0
    if !(card::is_level(d, c, &[7]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_attribute(d, c, ATTRIBUTE_WIND)) {
        return false;
    }
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    duel::get_location_count_from_ex(d, tp, Some(tp), h.into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=math.min(Duel.GetLocationCountFromEx(tp,nil,nil,TYPE_SYNCHRO),2)
    let mut ft = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::Type(TYPE_SYNCHRO), None).0.min(2);
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // ft=math.min(ft,aux.CheckSummonGate(tp) or ft)
    let gate = aux::check_summon_gate_limit(d, tp).unwrap_or(ft as i64);
    ft = ft.min(gate as i32);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
