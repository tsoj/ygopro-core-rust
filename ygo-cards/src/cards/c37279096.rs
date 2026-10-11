//! Ars Magna - "Citrinitas"
use super::prelude::*;

pub const ID: u32 = 37279096;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE, SET_DOOMZ, SET_ELFNOTE, SET_ARS_MAGNA, SET_POWER_PATRON, SET_MEDIUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // You can activate the effects of monsters you control with "Ars Magna" in their original names as Quick Effects while you control a "Power Patron" Link Monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_ARS_MAGNA_CITRINITAS);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(effcon)));
    effect::set_target_range(d, e1, 1, 0);
    card::register_effect(d, c, e1, false);
    // Once while face-up on the field: You can add 1 "Medius" or "Ars Magna" monster from your Deck to your hand. You can only use this effect of "Ars Magna - "Citrinitas"" once per turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn powerpatronfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&powerpatronfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MEDIUS as u16, SET_ARS_MAGNA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
