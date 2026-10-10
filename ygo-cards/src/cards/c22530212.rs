//! Magic Hand
use super::prelude::*;

pub const ID: u32 = 22530212;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_TO_HAND);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK) && !card::is_reason(d, c, REASON_DRAW)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, 1 - a.tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = group::filter(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, 1 - a.tp), NO_EX);
    duel::set_target_card(d, g.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 800);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
        if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX) {
            duel::damage(d, 1 - a.tp, 800, REASON_EFFECT, false, None);
        }
    }
}
