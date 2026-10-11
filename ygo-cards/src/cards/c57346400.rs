//! Guard Dog
use super::prelude::*;

pub const ID: u32 = 57346400;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_target_range(d, e1, 0, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
