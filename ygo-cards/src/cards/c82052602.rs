//! Gagagaback
use super::prelude::*;

pub const ID: u32 = 82052602;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GAGAGA], ..ScriptInfo::DEFAULT };

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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=false
        set_s_var(d, ID, 0, Value::Bool(false));
        // s[1]=false
        set_s_var(d, ID, 1, Value::Bool(false));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DESTROYED)
        effect::set_code(d, ge1, EVENT_BATTLE_DESTROYED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function() s[0]=false s[1]=false end)
        aux::add_values_reset(d, reset_flags);
    });
}
/// `aux.AddValuesReset(function() s[0]=false s[1]=false end)`
fn reset_flags(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Bool(false));
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // local pos=tc:GetPosition()
        let _pos = card::get_position(d, tc);
        // if tc:IsSetCard(SET_GAGAGA) and tc:IsLocation(LOCATION_GRAVE) and tc:IsReason(REASON_BATTLE)
        //   and tc:GetControler()==tc:GetPreviousControler() then
        if card::is_set_card(d, tc, &[SET_GAGAGA as u16])
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_reason(d, tc, REASON_BATTLE)
            && card::get_controler(d, tc) == card::get_previous_controler(d, tc)
        {
            // s[tc:GetControler()]=true
            let p = card::get_controler(d, tc);
            set_s_var(d, ID, p as u64, Value::Bool(true));
        }
    }
}
/// `s.filter(c,id,e,tp)`
fn filter(d: &mut Duel, c: CardId, turn: i64, e: EffectId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE) and c:GetTurnID()==id and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_reason(d, c, REASON_BATTLE)
        && card::get_turn_id(d, c) as i64 == turn
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return s[tp] and Duel.GetFlagEffect(tp,id)==0
    s_var(d, ID, a.tp as u64).truthy() && duel::get_flag_effect(d, a.tp, ID) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(...)
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        let turn = duel::get_turn_count(d, None) as i64;
        let (tp, e) = (a.tp, a.e);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, turn, e, tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let turn = duel::get_turn_count(d, None) as i64;
    let (tp, e) = (a.tp, a.e);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,ft,ft,nil,Duel.GetTurnCount(),e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, turn, e, tp)), a.tp, LOCATION_GRAVE, 0, ft as u16, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        let n = g.len();
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(tp,#g*600,REASON_EFFECT)
        duel::damage(d, a.tp, (n * 600) as i64, REASON_EFFECT, false, None);
    }
}
