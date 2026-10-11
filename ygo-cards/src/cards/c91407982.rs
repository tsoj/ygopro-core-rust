//! Fortune Vision
use super::prelude::*;

pub const ID: u32 = 91407982;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORTUNE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can add 1 "Fortune Lady" card from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Apply a "this turn, monsters you control cannot be destroyed by card effects" effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cannotdescon)));
    effect::set_target(d, e2, Some(cost_target(e2target)));
    effect::set_operation(d, e2, Some(operation(cannotdesop)));
    card::register_effect(d, c, e2, false);
    // Apply a "make the next battle damage you take this turn become 0" effect
    let e3 = effect::clone(d, e2);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_condition(d, e3, Some(condition(nodamcon)));
    effect::set_operation(d, e3, Some(operation(nodamop)));
    card::register_effect(d, c, e3, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_FORTUNE_LADY as u16]) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
fn confilter(d: &mut Duel, c: CardId, player: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_controler(d, c, player) && card::is_reason(d, c, REASON_EFFECT)
}
fn cannotdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, a.tp), 1, NO_EX) && !duel::has_flag_effect(d, a.tp, ID, None)
}
fn cannotdesop(d: &mut Duel, a: &EffArgs) {
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    let c = effect::get_handler(d, a.e).nil();
    // This turn, monsters you control cannot be destroyed by card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 4)), 0, None);
}
fn e2target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
fn nodamcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, 1 - a.tp), 1, NO_EX)
}
fn nodamop(d: &mut Duel, a: &EffArgs) {
    // Make the next battle damage you take this turn become 0
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 5));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
