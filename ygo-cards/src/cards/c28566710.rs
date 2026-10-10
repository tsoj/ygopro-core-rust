//! Last Turn
use super::prelude::*;

pub const ID: u32 = 28566710;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<=1000 and Duel.GetTurnPlayer()~=tp
    duel::get_lp(d, a.tp) <= 1000 && duel::get_turn_player(d) != a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tg=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
    let tg = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc=tg:GetFirst()
    let tc = group::get_first(d, &tg);
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD)
    let mut hg = duel::get_field_group(d, a.tp, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD);
    // if tc then hg:RemoveCard(tc) end
    if let Some(tc) = tc {
        group::remove_card(&mut hg, &CardSet::from_card(tc));
    }
    // Duel.SendtoGrave(hg,REASON_EFFECT)
    duel::send_to_grave(d, hg, REASON_EFFECT, None, None);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(1-tp,s.spfilter,1-tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), 1 - a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,1-tp,1-tp,false,false,POS_FACEUP_ATTACK)
        duel::special_summon(d, sc, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
        // e1:SetTargetRange(1,1)
        effect::set_target_range(d, e1, 1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // if tc then Duel.ForceAttack(sc,tc) end
        if let Some(tc) = tc {
            duel::force_attack(d, sc, tc);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // if Duel.IsPhase(PHASE_END) then
    if duel::is_phase(d, PHASE_END) {
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    }
    // e1:SetOperation(s.checkop)
    effect::set_operation(d, e1, Some(operation(checkop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, _a: &EffArgs) {
    // local t1=Duel.GetFieldGroupCount(0,LOCATION_MZONE,0)
    let t1 = duel::get_field_group_count(d, 0, LOCATION_MZONE, 0);
    // local t2=Duel.GetFieldGroupCount(1,LOCATION_MZONE,0)
    let t2 = duel::get_field_group_count(d, 1, LOCATION_MZONE, 0);
    // if t1>0 and t2==0 then
    if t1 > 0 && t2 == 0 {
        // Duel.Win(0,WIN_REASON_LAST_TURN)
        duel::win(d, 0, WIN_REASON_LAST_TURN);
    // elseif t2>0 and t1==0 then
    } else if t2 > 0 && t1 == 0 {
        // Duel.Win(1,WIN_REASON_LAST_TURN)
        duel::win(d, 1, WIN_REASON_LAST_TURN);
    } else {
        // Duel.Win(PLAYER_NONE,WIN_REASON_LAST_TURN)
        duel::win(d, PLAYER_NONE as u8, WIN_REASON_LAST_TURN);
    }
}
