//! Royal Penguins Garden
use super::prelude::*;

pub const ID: u32 = 80893872;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PENGUIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Reduce level then discard
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_LVCHANGE + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    effect::set_operation(d, e2, Some(operation(lvop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PENGUIN as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0) as u64) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &sg);
    }
}
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PENGUIN as u16]) && card::is_level_above(d, c, 2) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&lvfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT | REASON_DISCARD), None)),
                a.tp,
                LOCATION_HAND,
                0,
                1,
                NO_EX,
            );
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2) as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&lvfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::hint_selection(d, g.clone(), true);
        } else {
            duel::confirm_cards(d, 1 - a.tp, &g);
            duel::shuffle_hand(d, a.tp);
        }
        // Reduce level
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        effect::set_value(d, e1, -1);
        card::register_effect(d, tc, e1, false);
        if duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT | REASON_DISCARD), None)),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        ) {
            duel::break_effect(d);
            duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
}
