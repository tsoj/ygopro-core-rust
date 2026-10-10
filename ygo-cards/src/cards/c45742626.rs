//! Pilgrim Reaper
use super::prelude::*;

pub const ID: u32 = 45742626;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // atk/def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(value));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // deckdes
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DECKDES);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
}
fn value(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    duel::get_matching_group_count(d, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK)), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64 * 200
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5) && duel::is_player_can_discard_deck(d, 1 - a.tp, 5);
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 5);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let mut g1 = duel::get_decktop_group(d, a.tp, 5);
    let g2 = duel::get_decktop_group(d, 1 - a.tp, 5);
    group::add_card(&mut g1, &g2);
    duel::disable_shuffle_check(d, true);
    duel::send_to_grave(d, g1, REASON_EFFECT, None, None);
}
