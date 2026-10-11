//! Spirit Sculptor
use super::prelude::*;

pub const ID: u32 = 19307353;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.statsum(c)`
fn statsum(d: &mut Duel, c: CardId) -> i64 {
    (card::get_text_attack(d, c).max(0) + card::get_text_defense(d, c).max(0)) as i64
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    if !(card::is_attack_above(d, c, 0) && card::is_defense_above(d, c, 0)) {
        return false;
    }
    if !(card::get_text_attack(d, c) >= 0 && card::get_text_defense(d, c) >= 0) {
        return false;
    }
    let total = statsum(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, total)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,total)`
fn thfilter(d: &mut Duel, c: CardId, total: i64) -> bool {
    if !(card::get_text_attack(d, c) >= 0 && card::get_text_defense(d, c) >= 0) {
        return false;
    }
    let ctotal = statsum(d, c);
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && ctotal == total
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    let tc = group::get_first(d, &g).nil();
    let total = statsum(d, tc);
    effect::set_label(d, a.e, &[total]);
    duel::release(d, tc, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let total = effect::get_label(d, a.e);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, total)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
