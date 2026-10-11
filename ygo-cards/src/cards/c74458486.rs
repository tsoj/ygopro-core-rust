//! Covering Fire
use super::prelude::*;

pub const ID: u32 = 74458486;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}

/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // return Duel.IsPhase(PHASE_DAMAGE) and not Duel.IsDamageCalculated() and bc and bc:IsControler(tp) and bc:IsFaceup()
    duel::is_phase(d, PHASE_DAMAGE) && !duel::is_damage_calculated(d) && bc.is_some_and(|bc| card::is_controler(d, bc, a.tp) && card::is_faceup(d, bc))
}

/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:HasNonZeroAttack() and chkc~=bc end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::has_non_zero_attack(d, chkc)
            && Some(chkc) != bc;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.HasNonZeroAttack,tp,LOCATION_MZONE,0,1,bc) end
    let ex = bc.map_or(NO_EX, |b| b.into());
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::has_non_zero_attack), tp, LOCATION_MZONE, 0, 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.HasNonZeroAttack,tp,LOCATION_MZONE,0,1,1,bc)
    let _ = duel::select_target(d, tp, Some(&card::has_non_zero_attack), tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    true
}

/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d).nil();
    // if bc:IsFaceup() and bc:IsRelateToBattle() and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_faceup(d, bc)
        && card::is_relate_to_battle(d, bc)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_faceup(d, tc.nil())
        && card::is_controler(d, tc.nil(), a.tp)
    {
        // The attacked monster gains ATK equal to that target's ATK, during the Damage Step only
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(tc:GetAttack())
        let atk = card::get_attack(d, tc.nil());
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
}
