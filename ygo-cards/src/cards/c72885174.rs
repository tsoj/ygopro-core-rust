//! Slip of Fortune
use super::prelude::*;

pub const ID: u32 = 72885174;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
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
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()
    duel::is_turn_player(d, 1 - a.tp) && duel::get_attack_target(d).is_some()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if chk==0 then return a:IsOnField() and a:IsCanBeEffectTarget(e)
    //     and d:IsOnField() and d:IsAbleToRemove() and d:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let at = at.nil();
        let dt = dt.nil();
        return card::is_on_field(d, at)
            && card::is_can_be_effect_target(d, at, Some(e))
            && card::is_on_field(d, dt)
            && card::is_able_to_remove(d, dt, None, None, None)
            && card::is_can_be_effect_target(d, dt, Some(e));
    }
    // local g=Group.FromCards(a,d)
    let at = at.nil();
    let dt = dt.nil();
    let mut g = CardSet::from_card(at);
    group::add_card(&mut g, &CardSet::from_card(dt));
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(dt)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.NegateAttack()
    duel::negate_attack(d);
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // if a:IsRelateToEffect(e) and a:IsFaceup() and tc:IsRelateToEffect(e)
    //     and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    let at = at.nil();
    if card::is_relate_to_effect(d, at, e)
        && card::is_faceup(d, at)
        && card::is_relate_to_effect(d, tc.nil(), e)
        && duel::remove(d, tc.nil(), 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0
    {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 1);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
