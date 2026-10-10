//! Jigen Bakudan
use super::prelude::*;

pub const ID: u32 = 90020065;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetCost(Cost.SelfTribute)
        effect::set_cost(d, e1, Some(cost::self_tribute()));
        // e1:SetTarget(s.destg)
        effect::set_target(d, e1, Some(cost_target(destg)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // local dam=g:GetSum(Card.GetAttack)/2
    let sum = group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64);
    let dam = lua_int(sum as f64 / 2.0);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // Duel.Damage(1-tp,dam,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None);
}
