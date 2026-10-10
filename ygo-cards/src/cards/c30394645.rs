//! Dogmatika Lawbringer
use super::prelude::*;

pub const ID: u32 = 30394645;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| {
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
    })));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Make each player send 1 monster from their Extra Deck to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let handler = effect::get_handler(d, a.e).nil();
        card::is_summon_location(d, handler, LOCATION_HAND)
    })));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // Add 1 "Dogmatika" card from your GY to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, PLAYER_ALL as u8, LOCATION_EXTRA as i32);
    true
}
fn tgop(d: &mut Duel, _a: &EffArgs) {
    let mut sg = CardSet::new();
    for p in 0..2u8 {
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOGRAVE as u64);
        let sc = duel::select_matching_card(d, p, Some(&tgfilter), p, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sc) {
            group::add_card(&mut sg, &CardSet::from_card(sc));
        }
    }
    if !sg.is_empty() {
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
