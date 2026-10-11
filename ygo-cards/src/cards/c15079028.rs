//! World Soul - Carbon
use super::prelude::*;

pub const ID: u32 = 15079028;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(scon)));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(stg)));
    effect::set_operation(d, e1, Some(operation(sop)));
    card::register_effect(d, c, e1, false);
    // To Decktop
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(tdcon)));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_FAIRY) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn scon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:GetHandler():GetSummonType()&SUMMON_TYPE_SPECIAL==0
    let c = effect::get_handler(d, a.e).nil();
    (card::get_summon_type(d, c) & SUMMON_TYPE_SPECIAL) == 0
}
fn stg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND + CATEGORY_SEARCH, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn sop(d: &mut Duel, a: &EffArgs) {
    if !duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let tc = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_hand(d, tc.clone(), Some(a.tp), REASON_EFFECT, None);
    duel::confirm_cards(d, 1 - a.tp, tc);
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_FAIRY)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::get_previous_controler(d, c) == tp
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // eg:IsExists(s.tgfilter,1,e:GetHandler(),tp) and not eg:IsContains(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d: &mut Duel, c: CardId| tgfilter(d, c, a.tp), 1, h.into()) && !group::is_contains(&eg, h)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_able_to_deck(d, h) {
        return;
    }
    duel::send_to_deck(d, h, Some(a.tp), SEQ_DECKTOP as i32, REASON_EFFECT, None);
}
