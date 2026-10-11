//! Capshell
use super::prelude::*;

pub const ID: u32 = 76722334;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card if...
    // It was tributed
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_RELEASE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
    // Used as material for a fusion, synchro, or link summon
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_condition(d, e2, Some(condition(drcon)));
    card::register_effect(d, c, e2, false);
    // Detached and ended up in GY to activate an Xyz monster's effect
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(dtchcon)));
    card::register_effect(d, c, e3, false);
    // Detached and ended up banished to activate an Xyz monster's effect
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_REMOVE);
    card::register_effect(d, c, e4, false);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_FUSION+REASON_SYNCHRO+REASON_LINK)~=0
    (a.r & (REASON_FUSION + REASON_SYNCHRO + REASON_LINK)) != 0
}
/// `s.dtchcon(e,tp,eg,ep,ev,re,r,rp)`
fn dtchcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_COST) and re:IsActivated() and re:IsActiveType(TYPE_XYZ) and c:IsPreviousLocation(LOCATION_OVERLAY)
    if !card::is_reason(d, c, REASON_COST) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) {
        return false;
    }
    effect::is_active_type(d, re, TYPE_XYZ) && card::is_previous_location(d, c, LOCATION_OVERLAY)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
