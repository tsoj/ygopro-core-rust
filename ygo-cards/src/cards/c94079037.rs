//! Mine Mole
use super::prelude::*;

pub const ID: u32 = 94079037;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indes
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_value_fn(d, e1, lua_fn(valcon));
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    // redirect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_LEAVE_FIELD_REDIRECT);
    effect::set_condition(d, e3, Some(lua_fn(rmcon)));
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e3, false);
}
/// `s.valcon(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_BATTLE) != 0
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, h, LOCATION_GRAVE) || a.r != REASON_SYNCHRO {
        return false;
    }
    let rc = card::get_reason_card(d, h).nil();
    card::is_race(d, rc, RACE_BEAST)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.rmcon(e)`
fn rmcon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_faceup(d, c) && card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != card::get_controler(d, c)
}
