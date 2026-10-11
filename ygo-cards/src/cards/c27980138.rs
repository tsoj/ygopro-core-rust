//! Quill Pen of Gulldos
use super::prelude::*;

pub const ID: u32 = 27980138;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g1 = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let g2 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g1, 2, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g2, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if group::get_count(&tg) < 2 {
        return;
    }
    let dg = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    if group::get_count(&dg) != 2 {
        return;
    }
    let hg = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_ONFIELD), NO_EX);
    if duel::send_to_deck(d, dg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && group::is_exists(d, &dg, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), 2, NO_EX)
        && !hg.is_empty()
    {
        duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
    }
}
