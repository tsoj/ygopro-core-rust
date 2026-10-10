//! The Deep Grave
use super::prelude::*;

pub const ID: u32 = 17688543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster during your next Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE as u32, Some(TIMING_END_PHASE as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsHasEffect(EFFECT_SPSUMMON_CONDITION) then
    if !card::is_has_effect(d, c, EFFECT_SPSUMMON_CONDITION, None).is_empty() {
        // local sc = c:IsHasEffect(EFFECT_SPSUMMON_CONDITION)
        let sc = card::is_has_effect(d, c, EFFECT_SPSUMMON_CONDITION, None).first().copied().nil();
        // local v = sc:GetValue()
        let v = effect::get_value(d, sc);
        // if type(v) == 'function' and not v(sc,e,tp,SUMMON_TYPE_SPECIAL) then
        if let Value::Fn(f) = &v {
            if !f.call(d, &[Value::Effect(sc), Value::Effect(e), Value::Int(tp as i64), Value::Int(SUMMON_TYPE_SPECIAL as i64)]).truthy() {
                // return false
                return false;
            }
        }
        // elseif v == 0 then
        else if matches!(v, Value::Int(0)) {
            // return false
            return false;
        }
    }
    // return c:IsMonster() and (not c:IsHasEffect(EFFECT_REVIVE_LIMIT) or c:IsStatus(STATUS_PROC_COMPLETE))
    card::is_monster(d, c) && (card::is_has_effect(d, c, EFFECT_REVIVE_LIMIT, None).is_empty() || card::is_status(d, c, STATUS_PROC_COMPLETE))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,tp)
    let sg = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON as u64, sg, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local res=(Duel.IsPhase(PHASE_STANDBY) and Duel.IsTurnPlayer(tp)) and 2 or 1
    let res: i64 = if duel::is_phase(d, PHASE_STANDBY) && duel::is_turn_player(d, a.tp) { 2 } else { 1 };
    // local turn_asc=(Duel.GetCurrentPhase()<PHASE_STANDBY and Duel.IsTurnPlayer(tp)) and 0 or (Duel.IsTurnPlayer(tp)) and 2 or 1
    let turn_asc: i64 = if duel::get_current_phase(d) < PHASE_STANDBY && duel::is_turn_player(d, a.tp) {
        0
    } else if duel::is_turn_player(d, a.tp) {
        2
    } else {
        1
    };
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(turn_asc+Duel.GetTurnCount())
    let turn_count = duel::get_turn_count(d, None) as i64;
    effect::set_label(d, e1, &[turn_asc + turn_count]);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,res)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, res as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // tc:CreateEffectRelation(e1)
    card::create_effect_relation(d, tc, e1);
}
/// `s.spcon(e,tp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetLabel() and Duel.IsTurnPlayer(tp)
    let turn_count = duel::get_turn_count(d, None) as i64;
    turn_count == effect::get_label(d, a.e) && duel::is_turn_player(d, a.tp)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsLocation(LOCATION_GRAVE) and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_GRAVE) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
