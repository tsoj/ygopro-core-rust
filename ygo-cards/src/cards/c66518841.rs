//! Prideful Roar
use super::prelude::*;

pub const ID: u32 = 66518841;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    if card::is_controler(d, tc.nil(), 1 - a.tp) {
        tc = duel::get_attack_target(d);
    }
    // if not tc then return false end
    let Some(tc) = tc else {
        return false;
    };
    // local bc=tc:GetBattleTarget()
    // if tc and bc then
    match card::get_battle_target(d, tc) {
        Some(bc) => {
            // local dif=bc:GetAttack()-tc:GetAttack()
            let dif = card::get_attack(d, bc) - card::get_attack(d, tc);
            // e:SetLabel(dif)
            effect::set_label(d, a.e, &[dif as i64]);
            // return dif>0
            dif > 0
        }
        // else return false end
        None => false,
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,e:GetLabel()) end
    if a.chk == Some(0) {
        let label = effect::get_label(d, a.e) as u32;
        return duel::check_lp_cost(d, a.tp, label);
    }
    // Duel.PayLPCost(tp,e:GetLabel())
    let label = effect::get_label(d, a.e) as u32;
    duel::pay_lp_cost(d, a.tp, label);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    if card::is_controler(d, tc.nil(), 1 - a.tp) {
        tc = duel::get_attack_target(d);
    }
    let tc = tc.nil();
    // local bc=tc:GetBattleTarget()
    let bc = card::get_battle_target(d, tc).nil();
    // local dif=bc:GetAttack()-tc:GetAttack()
    let dif = card::get_attack(d, bc) - card::get_attack(d, tc);
    // if dif>0 and tc:IsRelateToBattle() and bc:IsRelateToBattle() and tc:IsFaceup() and bc:IsFaceup() then
    if dif > 0 && card::is_relate_to_battle(d, tc) && card::is_relate_to_battle(d, bc) && card::is_faceup(d, tc) && card::is_faceup(d, bc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(dif+300)
        effect::set_value(d, e1, (dif + 300) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
