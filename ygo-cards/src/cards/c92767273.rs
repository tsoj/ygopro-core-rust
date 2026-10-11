//! Performapal Barracuda
use super::prelude::*;

pub const ID: u32 = 92767273;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Make an opponent's monster that battles your "Performapal" monster lose ATK equal to the difference between its original ATK and current ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.oppatkcon)
    effect::set_condition(d, e1, Some(condition(oppatkcon)));
    // e1:SetOperation(s.oppatkop)
    effect::set_operation(d, e1, Some(operation(oppatkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make 1 "Performapal" monster whose current ATK is different from its original ATK gain ATK equal to the difference until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP as u32, None);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e2:SetTarget(s.ppalatktg)
    effect::set_target(d, e2, Some(cost_target(ppalatktg)));
    // e2:SetOperation(s.ppalatkop)
    effect::set_operation(d, e2, Some(operation(ppalatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.oppatkcon(e,tp,eg,ep,ev,re,r,rp)`
fn oppatkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    // return bc1 and bc1:IsSetCard(SET_PERFORMAPAL) and bc2 and not bc2:IsAttack(bc2:GetBaseAttack())
    let Some(bc1) = bc1 else {
        return false;
    };
    if !card::is_set_card(d, bc1, &[SET_PERFORMAPAL as u16]) {
        return false;
    }
    let Some(bc2) = bc2 else {
        return false;
    };
    let base = card::get_base_attack(d, bc2);
    !card::is_attack(d, bc2, &[base as u32])
}
/// `s.oppatkop(e,tp,eg,ep,ev,re,r,rp)`
fn oppatkop(d: &mut Duel, a: &EffArgs) {
    // local _,bc=Duel.GetBattleMonster(tp)
    let (_, bc) = duel::get_battle_monster(d, a.tp);
    let bc = bc.nil();
    // if bc:IsRelateToBattle() and bc:IsFaceup() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // local diff=math.abs(bc:GetBaseAttack()-bc:GetAttack())
        let diff = (card::get_base_attack(d, bc) - card::get_attack(d, bc)).abs();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-diff)
        effect::set_value(d, e1, -diff as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
/// `s.ppalatkfilter(c)`
fn ppalatkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL) and c:IsFaceup() and not c:IsAttack(c:GetBaseAttack())
    if !card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) || !card::is_faceup(d, c) {
        return false;
    }
    let base = card::get_base_attack(d, c);
    !card::is_attack(d, c, &[base as u32])
}
/// `s.ppalatktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ppalatktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.ppalatkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && ppalatkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ppalatkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ppalatkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,s.ppalatkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&ppalatkfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.ppalatkop(e,tp,eg,ep,ev,re,r,rp)`
fn ppalatkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local diff=math.abs(tc:GetBaseAttack()-tc:GetAttack())
        let diff = (card::get_base_attack(d, tc) - card::get_attack(d, tc)).abs();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(diff)
        effect::set_value(d, e1, diff as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
