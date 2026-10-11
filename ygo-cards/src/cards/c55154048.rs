//! Nordic Relic Draupnir
use super::prelude::*;

pub const ID: u32 = 55154048;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC, SET_AESIR, SET_NORDIC_RELIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), None, None, None, None, None, None);
    // Atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 800);
    card::register_effect(d, c, e2, false);
    // Search
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(thcon)));
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NORDIC as u16]) || card::is_set_card(d, c, &[SET_AESIR as u16])
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, h, POS_FACEUP)
        && (card::get_reason(d, h) & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NORDIC_RELIC as u16]) && card::is_able_to_hand(d, c, None)
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
