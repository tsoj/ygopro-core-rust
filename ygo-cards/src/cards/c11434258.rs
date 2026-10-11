//! Saryuja's Shackles
use super::prelude::*;

pub const ID: u32 = 11434258;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_link_monster(d, c)
        && {
            let link = card::get_link(d, c);
            duel::is_player_can_draw(d, tp, link)
        }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE)
            && card::is_controler(d, chkc, 1 - tp)
            && filter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    duel::set_target_player(d, tp);
    let tc = group::get_first(d, &g).nil();
    let link = card::get_link(d, tc);
    duel::set_target_param(d, link);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, link as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let tc = duel::first_target(d).nil();
    let ct = card::get_link(d, tc);
    if card::is_relate_to_effect(d, tc, a.e) && duel::draw(d, p, ct, REASON_EFFECT) == ct as i32 {
        let hdct = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        let g = duel::get_matching_group(d, Some(&card::is_able_to_deck), p, LOCATION_HAND, 0, NO_EX); // gets cards that can be return
        if hdct <= 1 || g.is_empty() {
            return; // if fewer than 2 OR cannot return
        }
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        let sg = group::select(d, &g, p, 2, 2, false, NO_EX).nil(); // selects
        duel::send_to_deck(d, &sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        let og = duel::get_operated_group(d);
        let ct2 = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
        if ct2 > 1 {
            duel::sort_deckbottom(d, p, p, 2);
        }
    }
}
