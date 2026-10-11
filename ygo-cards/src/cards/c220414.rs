//! Performapal Turn Trooper
use super::prelude::*;

pub const ID: u32 = 220414;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x14a)
    card::enable_counter_permit(d, c, 0x14a, None, None);
    // c:SetCounterLimit(0x14a,2)
    card::set_counter_limit(d, c, 0x14a, 2);
    // Place 1 counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.countercon)
    effect::set_condition(d, e1, Some(condition(countercon)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(function() Duel.NegateAttack() end)
    effect::set_operation(d, e2, Some(operation(negate_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish all monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():GetCounter(0x14a)==2 end)
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.countercon(e,tp,...)`
fn countercon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():GetCounter(0x14a)<2
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && card::get_counter(d, h, 0x14a) < 2
}
/// `s.counterop(e,tp,...)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(0x14a,1)
        card::add_counter(d, c, 0x14a, 1, false);
    }
}
/// `s.atkcon(e,tp,...)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and e:GetHandler():GetCounter(0x14a)==1
    let attacker = duel::get_attacker(d).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_controler(d, attacker, 1 - a.tp) && card::get_counter(d, h, 0x14a) == 1
}
/// `function() Duel.NegateAttack() end`
fn negate_op(d: &mut Duel, _a: &EffArgs) {
    duel::negate_attack(d);
}
/// `function(e) return e:GetHandler():GetCounter(0x14a)==2 end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x14a) == 2
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, tp, LOCATION_MZONE as i32);
    true
}
/// `function s.retcon(ag,e,tp,eg,ep,ev,re,r,rp)` (the `cond` of `aux.RemoveUntil`)
fn retcon(d: &mut Duel, _ag: &CardSet, a: &EffArgs) -> bool {
    // if Duel.IsTurnPlayer(tp) then return false end
    if duel::is_turn_player(d, a.tp) {
        return false;
    }
    // local fid,t=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let fid = labels.first().copied().unwrap_or(0);
    let t = labels.get(1).copied();
    // if t==1 then return true end
    if t == Some(1) {
        return true;
    }
    // e:SetLabel(fid,1)
    effect::set_label(d, a.e, &[fid, 1]);
    // return false
    false
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // aux.RemoveUntil(g,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp,s.retcon,RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
    aux::remove_until(
        d,
        g,
        None,
        REASON_EFFECT,
        PHASE_END,
        ID,
        e,
        tp,
        Some(aux::default_field_return()),
        Some(std::sync::Arc::new(retcon)),
        Some(RESET_PHASE | PHASE_END | RESET_OPPO_TURN),
        Some(2),
        None,
        None,
    );
}
