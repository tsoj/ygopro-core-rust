//! Giant Starfall
//! 巨星墜とし
use super::prelude::*;

pub const ID: u32 = 43986064;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E) as u32));
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:HasLevel()
    card::is_faceup(d, c) && !card::has_level(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
    true
}
/// `s.chainlm(re,rp,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsMonsterEffect() and not re:GetHandler():HasLevel())
    let re = a.e(0);
    let is_monster = effect::is_monster_effect(d, re);
    let h = effect::get_handler(d, re).nil();
    !(is_monster && !card::has_level(d, h))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Change ATK to 0
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);

        // Cannot activate its effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3302)
        effect::set_description(d, e2, 3302);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);

        // Cannot be destroyed by battle
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(3000)
        effect::set_description(d, e3, 3000);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);

        // Halve battle damage
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e4, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e4:SetValue(HALF_DAMAGE)
        effect::set_value(d, e4, HALF_DAMAGE as i64);
        // e4:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
