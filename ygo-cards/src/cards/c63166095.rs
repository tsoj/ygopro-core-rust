//! Sky Striker Mobilize - Engage!
use super::prelude::*;

pub const ID: u32 = 63166095;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) < 5
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
        if duel::is_player_can_draw(d, tp, 1)
            && duel::get_matching_group_count(d, Some(&|d, c| card::is_spell(d, c)), tp, LOCATION_GRAVE, 0, NO_EX) >= 3
            && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
        {
            duel::break_effect(d);
            duel::shuffle_deck(d, tp);
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
