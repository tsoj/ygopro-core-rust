//! The Graveyard in the Fourth Dimension
use super::prelude::*;

pub const ID: u32 = 88089103;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LV], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LV as u16]) && card::is_able_to_deck(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 2, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let e = a.e;
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    if !sg.is_empty() {
        duel::send_to_deck(d, &sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
