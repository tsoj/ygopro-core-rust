//! Subterror Succession
use super::prelude::*;

pub const ID: u32 = 73575650;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate (Send monster to GY to add flip monster)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
fn with_chk(a: &EffArgs, chk: Option<i64>) -> EffArgs {
    EffArgs { chk, ..a.clone() }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let chk0 = with_chk(a, Some(0));
    if a.chk == Some(0) {
        return thtg1(d, &chk0) || thtg2(d, &chk0);
    }
    if thtg1(d, &chk0) && thtg2(d, &chk0) {
        let opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
        if opt == 0 {
            thtg1(d, a);
        } else {
            thtg2(d, a);
        }
        effect::set_label(d, a.e, &[(opt + 1) as i64]);
    } else if thtg1(d, &chk0) {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]);
        thtg1(d, a);
        effect::set_label(d, a.e, &[1]);
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
        thtg2(d, a);
        effect::set_label(d, a.e, &[2]);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    if label == 1 {
        thop1(d, a);
    } else if label == 2 {
        thop2(d, a);
    }
}
/// Check for monster from hand or field to send to GY
fn tgfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let code = card::get_code(d, c).first().copied().unwrap_or(0);
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_monster(d, c)
        && card::is_able_to_grave(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter1(d, cc, c, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// Check for flip monster
fn thfilter1(d: &mut Duel, c: CardId, tc: CardId, code: u32) -> bool {
    card::is_monster(d, c)
        && card::is_type(d, c, TYPE_FLIP)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[code])
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc)
}
/// Activation legality
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter1(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// Performing the effect of adding to hand
fn thop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let tg = duel::select_matching_card(d, tp, Some(&|d, cc| tgfilter1(d, cc, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tg);
    if let Some(tc) = tc {
        let code = card::get_code(d, tc).first().copied().nil();
        if duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, cc| thfilter1(d, cc, tc, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - tp, &g);
            }
        }
    }
    set_back(d, a, c);
}
/// Check for monster from hand or field to send to GY
fn tgfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    ((card::is_location(d, c, LOCATION_HAND) && card::get_original_level(d, c) > 0) || card::is_faceup(d, c))
        && card::is_type(d, c, TYPE_FLIP)
        && card::is_able_to_grave(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter2(d, cc, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// Check for monster with same attribute but lower level
fn thfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && card::get_original_level(d, c) < card::get_original_level(d, tc)
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc)
}
/// Activation legality
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter2(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// Performing the effect of adding to hand
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let tg = duel::select_matching_card(d, tp, Some(&|d, cc| tgfilter2(d, cc, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &tg);
    if let Some(tc) = tc {
        if duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            let g = duel::select_matching_card(d, tp, Some(&|d, cc| thfilter2(d, cc, tc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - tp, &g);
            }
        }
    }
    set_back(d, a, c);
}
/// The shared end of both operations: the activated card is set again.
fn set_back(d: &mut Duel, a: &EffArgs, c: CardId) {
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, true, None) && effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        duel::break_effect(d);
        card::cancel_to_grave(d, c, true);
        duel::change_position(d, c, POS_FACEDOWN as u8, None, None, None, false);
        duel::raise_event(d, CardSet::from_card(c), EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
    }
}
