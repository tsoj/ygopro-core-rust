//! Reincarnation of the Unchained
use super::prelude::*;

pub const ID: u32 = 67803035;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UNCHAINED], listed_names: &[27412542, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can add 1 "Unchained" Spell (except "Reincarnation of the Unchained") or 1 "Abomination's Prison" from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Once per turn: You can target up to 3 "Unchained" cards in your GY, except "Reincarnation of the Unchained"; shuffle them into the Deck, then you can destroy other cards you control up to the number returned
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && card::is_spell(d, c) && !card::is_code(d, c, &[ID])) || card::is_code(d, c, &[27412542]))
        && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() && duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        duel::shuffle_deck(d, a.tp);
        let c = effect::get_handler(d, a.e).nil();
        let exc: Option<CardId> = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
        let operated = duel::get_operated_group(d);
        let returned_count = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
        let ex = exc.map_or(NO_EX, |c| c.into());
        if returned_count > 0 && duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, 0, 1, ex) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, 0, 1, returned_count as u16, false, ex).nil();
            if !g.is_empty() {
                duel::hint_selection(d, g.clone(), true);
                duel::break_effect(d);
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
