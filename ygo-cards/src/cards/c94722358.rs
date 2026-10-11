//! Maliss <C> MTP-07
use super::prelude::*;

pub const ID: u32 = 94722358;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALISS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Maliss" monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Can be activated the turn it was Set by banishing 1 face-up "Maliss" monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| {
        effect::set_label(d, a.e(0), &[1]);
        Value::Nil
    }));
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| {
        let p = effect::get_handler_player(d, a.e(0));
        duel::is_existing_matching_card(d, Some(&|d, c| thcostfilter(d, c, p)), p, LOCATION_MZONE, 0, 1, NO_EX)
    })));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn thcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_able_to_remove(d, c, Some(tp), None, None)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    if effect::get_label(d, label_obj) > 0 {
        effect::set_label(d, label_obj, &[0]);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| thcostfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_type(d, c, TYPE_LINK)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::send_to_hand(d, &g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    if !card::is_location(d, group::get_first(d, &g).nil(), LOCATION_HAND) {
        return;
    }
    duel::confirm_cards(d, 1 - tp, &g);
    duel::shuffle_hand(d, tp);
    let c = effect::get_handler(d, e).nil();
    let exc = if card::is_relate_to_effect(d, c, e) { Some(c) } else { None };
    let rg = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove(d, cc, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc.into());
    if !rg.is_empty()
        && duel::is_existing_matching_card(d, Some(&linkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg = group::select(d, &rg, tp, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::hint_selection(d, &sg, true);
            duel::break_effect(d);
            duel::remove(d, &sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
