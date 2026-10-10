//! Broww, Huntsman of Dark World
use super::prelude::*;

pub const ID: u32 = 79126789;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(drcon)));
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(e:GetHandler():GetPreviousControler())
    let c = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, c);
    effect::set_label(d, a.e, &[prev as i64]);
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND) and r&(REASON_DISCARD|REASON_EFFECT)==REASON_DISCARD|REASON_EFFECT
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    let label = effect::get_label(d, a.e);
    if a.tp == a.rp || (a.tp as i64) != label {
        duel::set_target_param(d, 1);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    } else {
        duel::set_target_param(d, 2);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    }
    true
}
/// `s.drop` (named `drop_op`: `drop` is the prelude's function)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
