//! Clear Effector
use super::prelude::*;

pub const ID: u32 = 58518520;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If sent to GY as synchro material, draw 1
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    effect::set_condition(d, e1, Some(condition(drcon)));
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
    // A synchro monster using this card cannot be destroyed by card effects
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_operation(d, e2, Some(operation(indop)));
    card::register_effect(d, c, e2, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,...)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.indcon(e,tp,eg,ep,ev,re,r,rp)`
fn indcon(_d: &mut Duel, a: &EffArgs) -> bool {
    a.r == REASON_SYNCHRO
}
/// `s.indop(e,...)`
fn indop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    // Cannot be destroyed by card effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3001);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, rc, e1, false);
}
