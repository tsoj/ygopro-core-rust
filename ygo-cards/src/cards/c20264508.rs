//! Thunder Short
use super::prelude::*;

pub const ID: u32 = 20264508;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)~=0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) != 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // local dam=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)*400
    let dam = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) as i64 * 400;
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    // local dam=Duel.GetFieldGroupCount(p,LOCATION_MZONE,0)*400
    let dam = duel::get_field_group_count(d, p, LOCATION_MZONE, 0) as i64 * 400;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
