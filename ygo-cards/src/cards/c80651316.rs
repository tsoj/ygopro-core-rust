//! Evolsaur Cerato
use super::prelude::*;

pub const ID: u32 = 80651316;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EVOLTILE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(aux::evospcon()));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 200);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_target(d, e2, Some(cost_target(schtg)));
    effect::set_operation(d, e2, Some(operation(schop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e2, false);
}
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_EVOLTILE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn schtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn schop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&sfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
