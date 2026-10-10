//! Wave-Motion Cannon
use super::prelude::*;

pub const ID: u32 = 38992735;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(reset)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    effect::set_target(d, e2, Some(cost_target(tg)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // turn
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE_START | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(turncount)));
    card::register_effect(d, c, e3, false);
}
fn turncount(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsTurnPlayer(1-tp) then return end
    if duel::is_turn_player(d, 1 - a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    let ct = card::get_turn_counter(d, c);
    // ct=ct+1
    let ct = ct + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
}
fn reset(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetHandler():SetTurnCounter(0)
    let c = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, c, 0);
    true
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetTurnCounter()>0 end
    if a.chk == Some(0) {
        return card::get_turn_counter(d, c) > 0;
    }
    // local dam=c:GetTurnCounter()*1000
    let dam = card::get_turn_counter(d, c) * 1000;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
