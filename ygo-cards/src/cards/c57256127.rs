//! Shipping Archfiend
use super::prelude::*;

pub const ID: u32 = 57256127;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can target any number of face-up monsters on the field, and declare 1 Type or Attribute; they become that Type or Attribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(changetg)));
    effect::set_operation(d, e1, Some(operation(changeop)));
    card::register_effect(d, c, e1, false);
    // If a monster(s) is Special Summoned to your opponent's field, while this card is in your Monster Zone (except during the Damage Step): You can target 1 monster on each field with the same Type and Attribute from each other; return them to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    effect::set_operation(d, e2, Some(operation(rthop)));
    card::register_effect(d, c, e2, false);
}
fn single_bit(p: u64) -> bool {
    (p & p.wrapping_sub(1)) == 0
}
/// `s.get_excluded_values(tg, Card.GetRace)`
fn excluded_races(d: &mut Duel, tg: &CardSet) -> u64 {
    let g = group::filter(d, tg, &|d, c| single_bit(card::get_race(d, c)), NO_EX);
    group::get_bitwise_or(d, &g, &|d, c| card::get_race(d, c) as i64)
}
/// `s.get_excluded_values(tg, Card.GetAttribute)`
fn excluded_attributes(d: &mut Duel, tg: &CardSet) -> u32 {
    let g = group::filter(d, tg, &|d, c| single_bit(card::get_attribute(d, c) as u64), NO_EX);
    group::get_bitwise_or(d, &g, &|d, c| card::get_attribute(d, c) as i64) as u32
}
fn changetg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        if !(card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc)) {
            return false;
        }
        let labels = effect::get_labels(d, a.e);
        let op = labels.first().copied().unwrap_or(0);
        let val = labels.get(1).copied().unwrap_or(0);
        return (op == 1 && card::is_race_except(d, chkc, val as u64, None, 0, None)) || (op == 2 && card::is_attribute_except(d, chkc, val as u32, None, 0, None));
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    let max_target_count = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let tg = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, max_target_count as u16, false, NO_EX).nil();
    let excluded_races = excluded_races(d, &tg);
    let excluded_attributes = excluded_attributes(d, &tg);
    let b1 = (excluded_races & RACE_ALL) != RACE_ALL;
    let b2 = (excluded_attributes & ATTRIBUTE_ALL) != ATTRIBUTE_ALL;
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    let mut decl_race_attr: i64 = 0;
    if op == Some(1) {
        decl_race_attr = duel::announce_race(d, a.tp, 1, RACE_ALL & !excluded_races) as i64;
    } else if op == Some(2) {
        decl_race_attr = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL & !excluded_attributes) as i64;
    }
    // (SelectEffect is not cancelable here, so `op` is never nil)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64), decl_race_attr]);
    true
}
fn changeop(d: &mut Duel, a: &EffArgs) {
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_faceup(d, c), NO_EX);
    if tg.is_empty() {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let decl_race_attr = labels.get(1).copied().unwrap_or(0);
    let eff_code = if op == 1 { EFFECT_CHANGE_RACE } else { EFFECT_CHANGE_ATTRIBUTE };
    for tc in group::iter(d, &tg) {
        // They become that Type or Attribute
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, eff_code);
        effect::set_value(d, e1, decl_race_attr);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = sg.len() == 2
        && group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 2
        && group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 1;
    (res, false)
}
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let g = duel::get_target_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_RTOHAND as u64), None, None, false)
        .unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, a.tp, 0);
    true
}
fn rthop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    !group::is_contains(&eg, c) && group::is_exists(d, &eg, &|d, cc| card::is_controler(d, cc, 1 - a.tp), 1, NO_EX)
}
