//! Twin-Headed Wolf
use super::prelude::*;

pub const ID: u32 = 88132637;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&|d, c2| card::is_faceup(d, c2) && card::is_race(d, c2, RACE_FIEND)), a.tp, LOCATION_MZONE, 0, 1, c.into())
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
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
