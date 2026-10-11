//! Cross Breed
use super::prelude::*;

pub const ID: u32 = 20862918;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_label(d, e1, &[0]);
    card::register_effect(d, c, e1, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
fn costfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| costfilter2(d, x, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, c.into())
}
fn costfilter2(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::get_original_race(d, c) == card::get_original_race(d, tc)
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc)
        && card::get_original_code(d, c) != card::get_original_code(d, tc)
        && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, Some(c), Some(tc))), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, tc1: Option<CardId>, tc2: Option<CardId>) -> bool {
    let tc1 = tc1.nil();
    card::get_original_race(d, c) == card::get_original_race(d, tc1)
        && card::get_original_attribute(d, c) == card::get_original_attribute(d, tc1)
        && card::get_original_code(d, c) != card::get_original_code(d, tc1)
        && card::get_original_code(d, c) != card::get_original_code(d, tc2.nil())
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_matching_card(d, Some(&|d, c| costfilter1(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    effect::set_label(d, a.e, &[0]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| costfilter1(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let g1first = group::get_first(d, &g1).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g2 = duel::select_matching_card(d, tp, Some(&|d, c| costfilter2(d, c, g1first, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, g1first.into()).nil();
    let mut g1 = g1;
    group::add_card(&mut g1, &g2);
    duel::set_target_card(d, g1.clone());
    duel::remove(d, g1, POS_FACEUP as u8, REASON_COST, None, None);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let tc1 = group::get_first(d, &tg);
    let tc2 = group::iter(d, &tg).get(1).copied();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tc1, tc2)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
