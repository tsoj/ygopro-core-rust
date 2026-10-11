//! Pinpoint Dash
use super::prelude::*;

pub const ID: u32 = 94431029;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_EXTRA as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g0 = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_EXTRA, 0, NO_EX);
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), 1 - tp, LOCATION_EXTRA, 0, NO_EX);
    if g0.is_empty() || g1.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let sel0 = group::select(d, &g0, tp, 1, 1, false, NO_EX).nil();
    let tc0 = group::get_first(d, &sel0).nil();
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_CONFIRM as u64);
    let sel1 = group::select(d, &g1, 1 - tp, 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &sel1).nil();
    duel::confirm_cards(d, 1 - tp, &CardSet::from_card(tc0));
    duel::confirm_cards(d, tp, &CardSet::from_card(tc1));
    if compare(d, tc0, tc1) {
        if duel::send_to_grave(d, &CardSet::from_card(tc1), REASON_EFFECT, None, None) != 0
            && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(tc0), None).0 > 0
            && duel::special_summon(d, CardSet::from_card(tc0), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
            && card::get_original_race(d, tc0) == card::get_original_race(d, tc1)
            && card::get_original_attribute(d, tc0) == card::get_original_attribute(d, tc1)
        {
            let diff = card::get_attack(d, tc1).max(0);
            if diff > 0 {
                let lp = duel::get_lp(d, 1 - tp);
                duel::set_lp(d, 1 - tp, lp - diff);
            }
        }
    } else if duel::send_to_grave(d, &CardSet::from_card(tc0), REASON_EFFECT, None, None) != 0
        && duel::get_location_count_from_ex(d, 1 - tp, Some(1 - tp), NO_EX, duel::ExtraCard::Card(tc1), None).0 > 0
    {
        duel::special_summon(d, CardSet::from_card(tc1), 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
fn compare(d: &mut Duel, c1: CardId, c2: CardId) -> bool {
    // local tps={TYPE_FUSION,TYPE_SYNCHRO,TYPE_XYZ,TYPE_LINK}
    for t in [TYPE_FUSION, TYPE_SYNCHRO, TYPE_XYZ, TYPE_LINK] {
        if card::is_type(d, c1, t) && card::is_type(d, c2, t) {
            return true;
        }
    }
    false
}
