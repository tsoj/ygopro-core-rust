//! Soul Servant
use super::prelude::*;

pub const ID: u32 = 23020408;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL, ID],
    listed_series: &[SET_PALLADIUM],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 card on top of the Deck from your hand, Deck, or GY, that is "Dark Magician" or mentions "Dark Magician" or "Dark Magician Girl"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Draw cards equal to the number of "Palladium" monsters, "Dark Magician", and/or "Dark Magician Girl" with different names
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, deck_count: i32) -> bool {
    !card::is_code(d, c, &[ID])
        && (card::is_code(d, c, &[CARD_DARK_MAGICIAN]) || card::lists_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]))
        && ((card::is_location(d, c, LOCATION_DECK) && deck_count > 1) || (!card::is_location(d, c, LOCATION_DECK) && card::is_able_to_deck(d, c)))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ct)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let necro = aux::necro_valley_filter(move |d, c| filter(d, c, ct));
    let g = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        if card::is_location(d, tc, LOCATION_DECK) {
            duel::shuffle_deck(d, tp);
            duel::move_to_deck_top(d, tc);
        } else {
            duel::hint_selection(d, tc, true);
            duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
        if !card::is_location(d, tc, LOCATION_EXTRA) {
            duel::confirm_decktop(d, tp, 1);
        }
    }
}
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
        && ((card::is_set_card(d, c, &[SET_PALLADIUM as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]))
}
/// `Duel.GetMatchingGroup(s.drfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil):GetClassCount(Card.GetCode)`
fn draw_count(d: &mut Duel, tp: u8) -> i64 {
    let g = duel::get_matching_group(d, Some(&drfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ct = draw_count(d, tp);
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, tp, ct as u32);
    }
    duel::set_target_player(d, tp);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let ct = draw_count(d, a.tp);
    duel::draw(d, p, ct as u32, REASON_EFFECT);
}
