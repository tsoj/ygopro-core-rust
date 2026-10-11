//! Scareclaw Decline
use super::prelude::*;

pub const ID: u32 = 32152870;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 "Primitive Planet Reichphobia" to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Add 1 "Scareclaw" card from your GY to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn reichfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[56063182]) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE)
            && card::is_controler(d, chkc, a.tp)
            && reichfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&reichfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&reichfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    let loc = card::get_location(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), 1, a.tp, loc as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_VISAS_STARFROST])),
            a.tp,
            LOCATION_ONFIELD,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_matching_card(d, Some(&posfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&posfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::break_effect(d);
            duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
fn thcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_matching_group_count(d, Some(&|d, c| card::is_defense_pos(d, c)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX) >= 3
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SCARECLAW as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
