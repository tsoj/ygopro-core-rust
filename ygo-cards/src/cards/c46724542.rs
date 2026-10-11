//! Cyber Dragon Sieger
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 46724542;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),2,2,s.lcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, sc, st, Some(tp)))),
        2,
        Some(2),
        Some(std::sync::Arc::new(|d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_CYBER_DRAGON)
    effect::set_value(d, e1, CARD_CYBER_DRAGON as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSummonCode,1,nil,lc,sumtype,tp,CARD_CYBER_DRAGON)
    group::is_exists(d, g, &|d, c| card::is_summon_code(d, c, Some(lc), sumtype, tp, &[CARD_CYBER_DRAGON]), 1, NO_EX)
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition() and e:GetHandler():GetAttackAnnouncedCount()<1
    let h = effect::get_handler(d, a.e).nil();
    duel::is_battle_phase(d, None)
        && aux::stat_change_damage_step_condition(d)
        && card::get_attack_announced_count(d, h) < 1
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackAbove(2100) and c:IsRace(RACE_MACHINE)
    card::is_faceup(d, c) && card::is_attack_above(d, c, 2100) && card::is_race(d, c, RACE_MACHINE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| filter(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(2100)
        effect::set_value(d, e1, 2100);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_NO_BATTLE_DAMAGE)
        effect::set_code(d, e3, EFFECT_NO_BATTLE_DAMAGE);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e4, EFFECT_AVOID_BATTLE_DAMAGE);
        // e4:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        // e4:SetValue(1)
        effect::set_value(d, e4, 1);
        // c:RegisterEffect(e4)
        card::register_effect(d, c, e4, false);
    }
}
