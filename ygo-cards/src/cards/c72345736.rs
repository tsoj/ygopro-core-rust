//! Six Samurai United
use super::prelude::*;

pub const ID: u32 = 72345736;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_BUSHIDO], listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_BUSHIDO)
    card::enable_counter_permit(d, c, COUNTER_BUSHIDO as u16, None, None);
    // c:SetCounterLimit(COUNTER_BUSHIDO,2)
    card::set_counter_limit(d, c, COUNTER_BUSHIDO as u16, 2);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(s.drcost)
    effect::set_cost(d, e4, Some(cost_target(drcost)));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // cannot link material
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetCode(EFFECT_CANNOT_BE_LINK_MATERIAL)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_LINK_MATERIAL);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.ctfilter,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| ctfilter(d, c), 1, NO_EX) {
        // e:GetHandler():AddCounter(COUNTER_BUSHIDO,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, COUNTER_BUSHIDO as u16, 1, false);
    }
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // local ct=e:GetHandler():GetCounter(COUNTER_BUSHIDO)
    let ct = card::get_counter(d, h, COUNTER_BUSHIDO as u16);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetCounter(COUNTER_BUSHIDO)>0 and Duel.IsPlayerCanDraw(tp,c:GetCounter(COUNTER_BUSHIDO)) end
    if a.chk == Some(0) {
        let n = card::get_counter(d, c, COUNTER_BUSHIDO as u16);
        return n > 0 && duel::is_player_can_draw(d, a.tp, n as u32);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(ct)
    duel::set_target_param(d, ct as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
