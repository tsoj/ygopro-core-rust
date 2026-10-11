//! Zebufera, Vaalmonican Hallow Heathen
use super::prelude::*;

pub const ID: u32 = 2815176;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_VAALMONICA],
    counter_list: &[COUNTER_RESONANCE as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 1 Effect Monster
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),1,1,nil,nil,s.splimit)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 1, Some(1), None, None, Some(lua_fn(splimit)));
    // Cannot be Link Summoned unless you have a Fiend Monster Card with 3 or more Resonance Counters in your Pendulum Zone
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(s.splimit)
    effect::set_value_fn(d, e0, lua_fn(splimit));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can only Special Summon "Zebufera, Vaalmonican Hallow Heathen" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Remove 3 Resonance Counters from your Pendulum Zone instead of a card you control being destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(function(e,_c) return s.repfilter(_c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        let c = a.c(1);
        let p = effect::get_handler_player(d, a.e(0));
        repfilter(d, c, p)
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply the effect of 1 "Valmonica" Normal Spell/Trap that is banished or in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.cptg)
    effect::set_target(d, e2, Some(cost_target(cptg)));
    // e2:SetOperation(s.cpop)
    effect::set_operation(d, e2, Some(operation(cpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return (st&SUMMON_TYPE_LINK)~=SUMMON_TYPE_LINK or Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsOriginalRace,RACE_FIEND),sp,LOCATION_PZONE,0,nil):GetSum(Card.GetCounter,COUNTER_RESONANCE)>=3
    let st = a.int(3) as u64;
    let sp = a.p(2);
    (st & SUMMON_TYPE_LINK as u64) != SUMMON_TYPE_LINK as u64 || {
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_original_race(d, c, RACE_FIEND)), sp, LOCATION_PZONE, 0, NO_EX);
        group::get_sum(d, &g, &|d, c| card::get_counter(d, c, COUNTER_RESONANCE as u16) as i64) >= 3
    }
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField() and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_controler(d, c, tp) && card::is_on_field(d, c) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and Duel.IsCanRemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_EFFECT) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX)
            && duel::is_can_remove_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, h, Some(96)) {
        // return Duel.RemoveCounter(tp,1,0,COUNTER_RESONANCE,3,REASON_EFFECT)
        return duel::remove_counter(d, a.tp, 1, 0, COUNTER_RESONANCE as u16, 3, REASON_EFFECT);
    }
    // else return false end
    false
}
// function s.cpfilter(c)
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and (c:IsNormalSpell() or c:IsNormalTrap())
    //   and c:IsFaceup() and c:CheckActivateEffect(false,true,false)
    card::is_set_card(d, c, &[SET_VAALMONICA as u16])
        && (card::is_normal_spell(d, c) || card::is_normal_trap(d, c))
        && card::is_faceup(d, c)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
// function s.cptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.cpfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && cpfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cpfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cpfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.cpfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&cpfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local eff=tc:CheckActivateEffect(false,true,false)
    let eff = card::check_activate_effect(d, tc, false, true, false).nil().0;
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET|eff:GetProperty())
    let (eff_prop, _) = effect::get_property(d, eff);
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET | eff_prop, 0);
    // local tg=eff:GetTarget()
    let tg = effect::get_target(d, eff);
    // if tg then
    if let Some(tg) = tg {
        // tg(e,tp,eg,ep,ev,re,r,rp,1)
        let _ = tg.call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), a.eg.clone().map_or(Value::Nil, Value::Group), Value::Int(a.ep as i64), Value::Int(a.ev as i64), a.re.map_or(Value::Nil, Value::Effect), Value::Int(a.r as i64), Value::Int(a.rp as i64), Value::Int(1)]);
    }
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
// function s.cpop(e,tp,eg,ep,ev,re,r,rp)
fn cpop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local eff=tc:CheckActivateEffect(false,true,false)
    // if not eff then return end
    let Some((eff, _)) = card::check_activate_effect(d, tc, false, true, false) else {
        return;
    };
    // local op=eff:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, eff) {
        let _ = op.call(d, &[Value::Effect(a.e), Value::Int(a.tp as i64), a.eg.clone().map_or(Value::Nil, Value::Group), Value::Int(a.ep as i64), Value::Int(a.ev as i64), a.re.map_or(Value::Nil, Value::Effect), Value::Int(a.r as i64), Value::Int(a.rp as i64)]);
    }
}
