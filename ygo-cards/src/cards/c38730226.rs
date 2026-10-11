//! The Agent of Wisdom - Mercury
use super::prelude::*;

pub const ID: u32 = 38730226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp,chk)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0
    duel::is_turn_player(d, 1 - a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.dtg)
    effect::set_target(d, e1, Some(cost_target(dtg)));
    // e1:SetOperation(s.dop)
    effect::set_operation(d, e1, Some(operation(dop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.dtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.dop(e,tp,eg,ep,ev,re,r,rp)`
fn dop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
