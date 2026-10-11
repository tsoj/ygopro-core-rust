//! S-Force Nightchaser
//! Ｓ－Ｆｏｒｃｅ ナイトチェイサー
use super::prelude::*;

pub const ID: u32 = 20515672;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 non-Link "S-Force" monster
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
    // Each of your opponent's monsters in the same column as one of your "S-Force" monsters cannot target the monsters in its same column for attacks
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1a, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1a, 0, LOCATION_MZONE);
    // e1a:SetTarget(function(e,c) if SForce.ColumnTarget(e,c) then e:SetLabelObject(c) return true end end)
    effect::set_target(d, e1a, Some(effect_card_fn(column_label_target)));
    // e1a:SetValue(function(e,c) local bc=e:GetLabelObject() return bc and bc:GetColumnGroup():IsContains(c) end)
    effect::set_value_fn(d, e1a, effect_card_fn(column_value));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // During the Main Phase (Quick Effect): You can target 1 "S-Force" monster you control; shuffle it into the Deck, then you can Special Summon 1 of your banished "S-Force" monsters. You can only use this effect of "S-Force Nightchaser" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,stype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_S_FORCE,lc,stype,tp) and not c:IsType(TYPE_LINK,lc,stype,tp)
    card::is_set_card_ctx(d, c, &[SET_S_FORCE as u16], lc, stype, Some(tp)) && !card::is_type_ctx(d, c, TYPE_LINK, lc, stype, Some(tp))
}
/// `function(e,c) if SForce.ColumnTarget(e,c) then e:SetLabelObject(c) return true end end`
fn column_label_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    if aux::sforce::column_target().call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
        effect::set_label_object(d, e, Value::Card(c));
        return true;
    }
    false
}
/// `function(e,c) local bc=e:GetLabelObject() return bc and bc:GetColumnGroup():IsContains(c) end`
fn column_value(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    match effect::get_label_object(d, e).as_card() {
        Some(bc) => {
            let cg = card::get_column_group(d, bc, 0, 0);
            group::is_contains(&cg, c)
        }
        None => false,
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_S_FORCE as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_S_FORCE) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_S_FORCE as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
