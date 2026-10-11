//! Searchlightman
use super::prelude::*;

pub const ID: u32 = 67646312;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // cannot set
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_MSET);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_CANNOT_SSET);
    duel::register_effect(d, e2, tp);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EFFECT_CANNOT_TURN_SET);
    duel::register_effect(d, e3, tp);
    let e4 = effect::clone(d, e1);
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target(d, e4, Some(lua_fn(sumlimit)));
    duel::register_effect(d, e4, tp);
    aux::register_client_hint(d, h, 0, tp, 0, 1, Some(aux::stringid(ID, 1)), 0, None);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(_d: &mut Duel, a: Args) -> bool {
    let sumpos = a.int(4);
    (sumpos & POS_FACEDOWN as i64) > 0
}
