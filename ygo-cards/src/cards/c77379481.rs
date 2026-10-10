//! Sasuke Samurai #3
use super::prelude::*;

pub const ID: u32 = 77379481;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // local ht=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ht = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if ht<7 then
    if ht < 7 {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,7-ht)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, 7 - ht);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ht=Duel.GetFieldGroupCount(p,LOCATION_HAND,0)
    let ht = duel::get_field_group_count(d, p, LOCATION_HAND, 0);
    // if ht<7 then
    if ht < 7 {
        // Duel.Draw(p,7-ht,REASON_EFFECT)
        duel::draw(d, p, (7 - ht) as u32, REASON_EFFECT);
    }
}
