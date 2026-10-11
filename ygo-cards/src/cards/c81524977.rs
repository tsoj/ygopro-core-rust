//! Seed Cannon
use super::prelude::*;

pub const ID: u32 = 81524977;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x20], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x20)
    card::enable_counter_permit(d, c, 0x20, None, None);
    // c:SetCounterLimit(0x20,5)
    card::set_counter_limit(d, c, 0x20, 5);
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e5, CATEGORY_DAMAGE);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCost(s.damcost)
    effect::set_cost(d, e5, Some(cost_target(damcost)));
    // e5:SetTarget(s.damtg)
    effect::set_target(d, e5, Some(cost_target(damtg)));
    // e5:SetOperation(s.damop)
    effect::set_operation(d, e5, Some(operation(damop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsRace,RACE_PLANT),1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT), 1, NO_EX)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x20,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x20, 1, false);
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // e:SetLabel(e:GetHandler():GetCounter(0x20))
    let ct = card::get_counter(d, h, 0x20) as i64;
    effect::set_label(d, e, &[ct]);
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():GetCounter(0x20)>0 end
    if a.chk == Some(0) {
        return card::get_counter(d, h, 0x20) > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(e:GetLabel()*500)
    let amount = effect::get_label(d, e) * 500;
    duel::set_target_param(d, amount as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel()*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, (1 - tp) as u8, amount as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
