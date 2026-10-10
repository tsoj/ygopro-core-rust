//! Volcanic Shell
use super::prelude::*;

pub const ID: u32 = 33365932;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[33365932],
    ..ScriptInfo::DEFAULT
};

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, h, LOCATION_GRAVE) {
        return;
    }
    if let Some(tc) = duel::get_first_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
