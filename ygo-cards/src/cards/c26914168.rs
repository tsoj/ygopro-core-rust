//! Dark Honest
use super::prelude::*;

pub const ID: u32 = 26914168;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.rtg)
    effect::set_target(d, e1, Some(cost_target(rtg)));
    // e1:SetOperation(s.rop)
    effect::set_operation(d, e1, Some(operation(rop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP as u32, None);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetOperation(s.aop)
    effect::set_operation(d, e2, Some(operation(aop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// Return to hand
// function s.rtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_MZONE as i32);
    true
}
fn rop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
// ATK down
// function s.con(e,tp,eg,ep,ev,re,r,rp)
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return d~=nil and d:IsFaceup() and ((a:IsControler(tp) and a:IsAttribute(ATTRIBUTE_DARK) and a:IsRelateToBattle())
    //     or (d:IsControler(tp) and d:IsAttribute(ATTRIBUTE_DARK) and d:IsRelateToBattle()))
    let Some(dt) = dt else {
        return false;
    };
    if !card::is_faceup(d, dt) {
        return false;
    }
    let atk_side = {
        let atk = atk.nil();
        card::is_controler(d, atk, a.tp) && card::is_attribute(d, atk, ATTRIBUTE_DARK) && card::is_relate_to_battle(d, atk)
    };
    atk_side
        || (card::is_controler(d, dt, a.tp) && card::is_attribute(d, dt, ATTRIBUTE_DARK) && card::is_relate_to_battle(d, dt))
}
fn aop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d).nil();
    // if not a:IsRelateToBattle() or not d:IsRelateToBattle() then return end
    if !card::is_relate_to_battle(d, atk) || !card::is_relate_to_battle(d, dt) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e1, a.tp);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // if a:IsControler(tp) then
    if card::is_controler(d, atk, a.tp) {
        // e1:SetValue(-d:GetAttack())
        let v = -(card::get_attack(d, dt) as i64);
        effect::set_value(d, e1, v);
        // d:RegisterEffect(e1)
        card::register_effect(d, dt, e1, false);
    } else {
        // e1:SetValue(-a:GetAttack())
        let v = -(card::get_attack(d, atk) as i64);
        effect::set_value(d, e1, v);
        // a:RegisterEffect(e1)
        card::register_effect(d, atk, e1, false);
    }
}
