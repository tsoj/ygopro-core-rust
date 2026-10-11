//! Battle of the Elements
use super::prelude::*;

pub const ID: u32 = 60530944;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Players send monsters to the GY until they control only 1 attribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
fn getattr(d: &mut Duel, g: &CardSet) -> u32 {
    let mut aat = 0;
    for tc in group::iter(d, g) {
        aat |= card::get_attribute(d, tc);
    }
    aat
}
fn rmfilter(d: &mut Duel, c: CardId, at: u32) -> bool {
    card::get_attribute(d, c) == at
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        || duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        || duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) == 0
        || duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        return;
    }
    let mut g1 = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    let mut g2 = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    let _c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    let avail1 = getattr(d, &g1);
    let r1 = duel::announce_attribute(d, tp, 1, avail1);
    group::remove(d, &mut g1, &|d, c| rmfilter(d, c, r1), NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 0));
    let avail2 = getattr(d, &g2);
    let r2 = duel::announce_attribute(d, 1 - tp, 1, avail2);
    group::remove(d, &mut g2, &|d, c| rmfilter(d, c, r2), NO_EX);
    duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(tp));
    duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
}
