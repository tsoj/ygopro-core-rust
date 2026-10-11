//! Karakuri Anatomy
use super::prelude::*;

pub const ID: u32 = 85541675;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KARAKURI], counter_place_list: &[0x12], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x12)
    card::enable_counter_permit(d, c, 0x12, None, None);
    // c:SetCounterLimit(0x12,2)
    card::set_counter_limit(d, c, 0x12, 2);
    // activate
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
    // e2:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e2, EVENT_CHANGE_POS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.accon)
    effect::set_condition(d, e2, Some(condition(accon)));
    // e2:SetOperation(s.acop)
    effect::set_operation(d, e2, Some(operation(acop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCost(s.drcost)
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // local np=c:GetPosition()
    let np = card::get_position(d, c);
    // local pp=c:GetPreviousPosition()
    let pp = card::get_previous_position(d, c);
    // return c:IsSetCard(SET_KARAKURI) and ((pp==POS_FACEUP_ATTACK and np==POS_FACEUP_DEFENSE) or (pp==POS_FACEUP_DEFENSE and np==POS_FACEUP_ATTACK) or (pp==POS_FACEDOWN_DEFENSE and np==POS_FACEUP_ATTACK))
    card::is_set_card(d, c, &[SET_KARAKURI as u16])
        && ((pp == POS_FACEUP_ATTACK && np == POS_FACEUP_DEFENSE) || (pp == POS_FACEUP_DEFENSE && np == POS_FACEUP_ATTACK) || (pp == POS_FACEDOWN_DEFENSE && np == POS_FACEUP_ATTACK))
}
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
fn acop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x12,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x12, 1, false);
}
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, h);
    }
    // local ct=e:GetHandler():GetCounter(0x12)
    let ct = card::get_counter(d, h, 0x12);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    duel::send_to_grave(d, h, REASON_COST, None, None);
    true
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return c:GetCounter(0x12)>0 and Duel.IsPlayerCanDraw(tp,c:GetCounter(0x12))
        return card::get_counter(d, c, 0x12) > 0 && duel::is_player_can_draw(d, a.tp, card::get_counter(d, c, 0x12) as u32);
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
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
