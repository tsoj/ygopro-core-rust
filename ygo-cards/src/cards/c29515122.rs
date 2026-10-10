//! Digvorzhak, King of Heavy Industry
use super::prelude::*;

pub const ID: u32 = 29515122;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, 1 - a.tp, 3);
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, 3);
    true
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, 1 - a.tp, 3, REASON_EFFECT);
    let g = duel::get_operated_group(d);
    let ct = group::filter_count(d, &g, &cfilter, NO_EX) as u16;
    if ct == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let dg = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, ct, false, NO_EX).nil();
    duel::hint_selection(d, dg.clone(), true);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
