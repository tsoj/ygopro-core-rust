//! 波動再生
//! Reanimation Wave
use super::prelude::*;

pub const ID: u32 = 60312997;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()==nil and eg:GetFirst():IsLocation(LOCATION_MZONE)
    if !duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    if duel::get_attack_target(d).is_some() {
        return false;
    }
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_location(d, first, LOCATION_MZONE)
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsLevelBelow(lv) and c:IsType(TYPE_SYNCHRO) and c:IsStatus(STATUS_PROC_COMPLETE)
    card::is_level_below(d, c, lv) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_status(d, c, STATUS_PROC_COMPLETE)
}
/// `eg:GetFirst():GetLevel()`
fn attacker_level(d: &mut Duel, a: &EffArgs) -> u32 {
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::get_level(d, first)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        let lv = attacker_level(d, a);
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, lv);
    }
    let lv = attacker_level(d, a);
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,eg:GetFirst():GetLevel())
    duel::select_target(d, tp, Some(&|d, x| filter(d, x, lv)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(HALF_DAMAGE)
    effect::set_value(d, e1, HALF_DAMAGE as i64);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_DAMAGE_STEP_END)
            effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
            // e2:SetOperation(s.spop)
            effect::set_operation(d, e2, Some(operation(spop)));
            // e2:SetLabelObject(tc)
            effect::set_label_object(d, e2, Value::Card(tc));
            // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, tp);
        }
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
