//! Cauldron of the Old Man
use super::prelude::*;

pub const ID: u32 = 91740879;
/// `local COUNTER_CAULDRON=0x14e`
const COUNTER_CAULDRON: u16 = 0x14e;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_CAULDRON)
    card::enable_counter_permit(d, c, COUNTER_CAULDRON, None, None);
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Recover LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetCost(s.announcecost)
    effect::set_cost(d, e3, Some(cost_target(announcecost)));
    // e3:SetTarget(s.rcvtg)
    effect::set_target(d, e3, Some(cost_target(rcvtg)));
    // e3:SetOperation(s.rcvop)
    effect::set_operation(d, e3, Some(operation(rcvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Inflict Damage
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetTarget(s.dmgtg)
    effect::set_target(d, e4, Some(cost_target(dmgtg)));
    // e4:SetOperation(s.dmgop)
    effect::set_operation(d, e4, Some(operation(dmgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsCanAddCounter(tp,COUNTER_CAULDRON,1,c) end
    if a.chk == Some(0) {
        return duel::is_can_add_counter(d, a.tp, Some((COUNTER_CAULDRON, 1, c)));
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(COUNTER_CAULDRON,1)
        card::add_counter(d, c, COUNTER_CAULDRON, 1, false);
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.announcecost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn announcecost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
/// `s.rcvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rcvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetCounter(COUNTER_CAULDRON)>0 end
    if a.chk == Some(0) {
        return card::get_counter(d, c, COUNTER_CAULDRON) > 0;
    }
    // local val=c:GetCounter(COUNTER_CAULDRON)*500
    let val = card::get_counter(d, c, COUNTER_CAULDRON) * 500;
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,val)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, val);
    true
}
/// `s.rcvop(e,tp,eg,ep,ev,re,r,rp)`
fn rcvop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local d=e:GetHandler():GetCounter(COUNTER_CAULDRON)*500
    let amount = card::get_counter(d, h, COUNTER_CAULDRON) * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, p, amount as i64, REASON_EFFECT, false, None);
}
/// `s.dmgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dmgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetCounter(COUNTER_CAULDRON)>0 end
    if a.chk == Some(0) {
        return card::get_counter(d, c, COUNTER_CAULDRON) > 0;
    }
    // local dam=c:GetCounter(COUNTER_CAULDRON)*300
    let dam = card::get_counter(d, c, COUNTER_CAULDRON) * 300;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.dmgop(e,tp,eg,ep,ev,re,r,rp)`
fn dmgop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local d=e:GetHandler():GetCounter(COUNTER_CAULDRON)*300
    let amount = card::get_counter(d, h, COUNTER_CAULDRON) * 300;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, amount as i64, REASON_EFFECT, false, None);
}
