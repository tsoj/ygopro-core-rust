//! Blade Knight
use super::prelude::*;

pub const ID: u32 = 39507162;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 400);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) <= 1
}
/// `s.discon(e)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e);
    !duel::is_existing_matching_card(d, None, p, LOCATION_MZONE, 0, 1, h.map_or(NO_EX, |h| h.into()))
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // if bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and bc:IsType(TYPE_FLIP) then
    if let Some(bc) = bc {
        if card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && card::is_type(d, bc, TYPE_FLIP) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, bc, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
            card::register_effect(d, bc, e2, false);
        }
    }
}
