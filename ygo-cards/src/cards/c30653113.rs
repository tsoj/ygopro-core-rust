//! Blessings of the Nile
use super::prelude::*;

pub const ID: u32 = 30653113;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // recover
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e2, CATEGORY_RECOVER);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_HAND) and c:IsControler(tp) and c:IsReason(REASON_EFFECT) and c:IsReason(REASON_DISCARD)
    card::is_previous_location(d, c, LOCATION_HAND)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_reason(d, c, REASON_DISCARD)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and eg:IsExists(s.cfilter,1,nil,tp)
    if a.rp == a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 1000);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
