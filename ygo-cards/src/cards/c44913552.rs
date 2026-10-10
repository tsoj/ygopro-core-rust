//! Timeater
use super::prelude::*;

pub const ID: u32 = 44913552;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // skip draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(skipcon)));
    effect::set_operation(d, e2, Some(operation(skipop)));
    card::register_effect(d, c, e2, false);
}
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_MZONE)
}
fn skipop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_code(d, e1, EFFECT_SKIP_M1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_MAIN1 | RESET_OPPO_TURN, 1);
    duel::register_effect(d, e1, a.tp);
}
