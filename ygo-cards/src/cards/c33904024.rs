//! Shard of Greed
use super::prelude::*;

pub const ID: u32 = 33904024;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0xd], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0xd)
    card::enable_counter_permit(d, c, 0xd, None, None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DRAW);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // draw
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(drtg)));
    effect::set_operation(d, e3, Some(operation(drop)));
    card::register_effect(d, c, e3, false);
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if ep==tp and r==REASON_RULE then
    if a.ep == a.tp && a.r == REASON_RULE {
        // e:GetHandler():AddCounter(0xd,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0xd, 1, false);
    }
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0xd)>=2
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0xd) >= 2
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
