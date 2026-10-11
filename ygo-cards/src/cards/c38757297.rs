//! Photon Leo
use super::prelude::*;

pub const ID: u32 = 38757297;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - tp, 0)
            && duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    if g.is_empty() {
        return;
    }
    duel::send_to_deck(d, &g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    duel::shuffle_deck(d, 1 - tp);
    duel::break_effect(d);
    duel::draw(d, 1 - tp, g.len() as u32, REASON_EFFECT);
}
