//! Altergeist Silquitous
use super::prelude::*;

pub const ID: u32 = 89538537;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(rthcost)));
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    effect::set_operation(d, e1, Some(operation(rthop)));
    card::register_effect(d, c, e1, false);
    // to hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}

fn rthcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_able_to_hand_as_cost(d, c)
}

fn rthcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rthcfilter), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&rthcfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
    duel::send_to_hand(d, g, None, REASON_COST, None);
    true
}

fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_able_to_hand(d, chkc, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}

fn rthop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::get_first_target(d).first().copied() {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}

fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}

fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_trap(d, c) && card::is_able_to_hand(d, c, None)
}

fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let sg = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}

fn thop(d: &mut Duel, a: &EffArgs) {
    if let Some(tc) = duel::get_first_target(d).first().copied() {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
