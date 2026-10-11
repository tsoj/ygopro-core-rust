//! Phantasm Spiral Power
use super::prelude::*;

pub const ID: u32 = 61397885;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], listed_series: &[SET_PHANTASM_SPIRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Decrease the ATK/DEF of a monster by 1000 and negate its effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(function() return Duel.IsEnvironment(CARD_UMI) end)
    effect::set_condition(d, e2, Some(lua_fn(env_umi)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equip 1 "Phantasm Spiral" Equip Spell to a Normal Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.eqtg)
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    // e3:SetOperation(s.eqop)
    effect::set_operation(d, e3, Some(operation(eqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsType(TYPE_NORMAL)
    card::is_facedown(d, c) || !card::is_type(d, c, TYPE_NORMAL)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0
        // and not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
        && !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        // and aux.StatChangeDamageStepCondition()
        && aux::stat_change_damage_step_condition(d)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.mfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && mfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.mfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&mfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.mfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&mfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Decrease ATK/DEF by 1000
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(-1000)
        effect::set_value(d, e1, -1000);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Negate its effects
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e3, EFFECT_DISABLE);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e4, EFFECT_DISABLE_EFFECT);
        // e4:SetValue(RESET_TURN_SET)
        effect::set_value(d, e4, RESET_TURN_SET as i64);
        // e4:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
// function s.efilter(c,tp)
fn efilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL) and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_NORMAL)
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX)
}
// function s.eqfilter(c,tc)
fn eqfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsEquipSpell() and c:IsSetCard(SET_PHANTASM_SPIRAL) and c:CheckEquipTarget(tc)
    card::is_equip_spell(d, c) && card::is_set_card(d, c, &[SET_PHANTASM_SPIRAL as u16]) && card::check_equip_target(d, c, tc)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.efilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && efilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            // and Duel.IsExistingTarget(s.efilter,tp,LOCATION_MZONE,0,1,nil,tp) end
            && duel::is_existing_target(d, Some(&|d, c| efilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.efilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| efilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = tc else { return };
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil,tc)
    let nvf = aux::necro_valley_filter(|d, c| eqfilter(d, c, tc));
    let g = duel::select_matching_card(d, a.tp, Some(&nvf), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX);
    // local eq=g:GetFirst()
    let eq = g.and_then(|g| group::get_first(d, &g));
    // if eq then
    if let Some(eq) = eq {
        // Duel.Equip(tp,eq,tc,true)
        duel::equip(d, a.tp, eq, tc, true, false);
    }
}
// Duel.IsEnvironment(CARD_UMI) as a bare condition
fn env_umi(d: &mut Duel, _a: Args) -> bool {
    duel::is_environment(d, CARD_UMI, None, None)
}
