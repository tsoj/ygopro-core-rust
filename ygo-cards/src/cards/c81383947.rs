//! White Magician Pikeru
use super::prelude::*;

pub const ID: u32 = 81383947;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.target`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,ct*400)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, ct * 400);
    true
}
/// `s.operation`
fn op(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    // Duel.Recover(p,ct*400,REASON_EFFECT)
    duel::recover(d, p, (ct * 400) as i64, REASON_EFFECT, false, None);
}
