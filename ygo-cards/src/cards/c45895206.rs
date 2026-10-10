//! Dark-Piercing Light
use super::prelude::*;

pub const ID: u32 = 45895206;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::change_position(
            d,
            g,
            POS_FACEUP_ATTACK as u8,
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_DEFENSE as u8),
            Some(POS_FACEUP_DEFENSE as u8),
            false,
        );
    }
}
