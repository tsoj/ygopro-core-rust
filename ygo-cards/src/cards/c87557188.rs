//! The Stern Mystic
use super::prelude::*;

pub const ID: u32 = 87557188;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    if !g.is_empty() {
        duel::confirm_cards(d, a.tp, g);
    }
}
