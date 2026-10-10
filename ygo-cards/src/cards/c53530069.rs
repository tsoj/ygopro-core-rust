//! Spirit of the Breeze
use super::prelude::*;

pub const ID: u32 = 53530069;

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
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and e:GetHandler():IsAttackPos()
    a.tp == duel::get_turn_player(d) && card::is_attack_pos(d, effect::get_handler(d, a.e).nil())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, amount) = (info[0].int() as u8, info[1].int());
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsPosition(POS_FACEUP_ATTACK) and c:IsRelateToEffect(e) then
    if card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_relate_to_effect(d, c, a.e) {
        duel::recover(d, p, amount, REASON_EFFECT, false, None);
    }
}
