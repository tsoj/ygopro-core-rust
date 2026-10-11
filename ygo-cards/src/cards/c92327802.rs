//! Defect Compiler
use super::prelude::*;

pub const ID: u32 = 92327802;
const COUNTER_DEFECT: u16 = 0x43;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_DEFECT as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_DEFECT)
    // (a monster: the counter can be placed on the Monster Zone, the default of the core)
    card::enable_counter_permit(d, c, COUNTER_DEFECT, Some(LOCATION_MZONE), None);
    // c:SetCounterLimit(COUNTER_DEFECT,1)
    card::set_counter_limit(d, c, COUNTER_DEFECT, 1);
    // damage reduce
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e4, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.damval(e,re,val,r,rp,rc)
fn damval(d: &mut Duel, a: Args) -> i64 {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if r&REASON_EFFECT~=0 and c:IsCanAddCounter(COUNTER_DEFECT,1) and c:GetFlagEffect(id)==0 then
    if r & REASON_EFFECT != 0 && card::is_can_add_counter(d, c, COUNTER_DEFECT, 1, false, 0) && card::get_flag_effect(d, c, ID) == 0 {
        // c:AddCounter(COUNTER_DEFECT,1)
        card::add_counter(d, c, COUNTER_DEFECT, 1, false);
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // return 0
        return 0;
    }
    // return val
    val
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_DEFECT,1,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_DEFECT, 1, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_DEFECT,1,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, COUNTER_DEFECT, 1, REASON_COST);
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_CYBERSE)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_CYBERSE)
}
// function s.tg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.op(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(800)
        effect::set_value(d, e1, 800);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
