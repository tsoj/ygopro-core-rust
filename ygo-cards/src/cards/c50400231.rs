//! Satellite Cannon
use super::prelude::*;

pub const ID: u32 = 50400231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ind
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value_fn(d, e1, effect_card_fn(indval));
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    // atk clear
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_BATTLED);
    effect::set_operation(d, e3, Some(operation(retop)));
    card::register_effect(d, c, e3, false);
}
fn indval(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_level_below(d, c, 7)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 1000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if Some(c) == duel::get_attacker(d) {
        card::reset_effect(d, c, RESET_DISABLE, RESET_EVENT);
    }
}
