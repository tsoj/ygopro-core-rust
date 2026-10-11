//! Mimighoul Master
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 55537983;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle or card effects while your opponent controls any face-down monsters or you control a "Mimighoul" monster other than "Mimighoul Master"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(lua_fn(indescon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e2, false);
    // Add 1 "Mimighoul" monster from your Deck to your hand, except "Mimighoul Master"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SEARCH | CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
    // Change 1 face-down monster on the field to face-up Attack or Defense Position
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_POSITION);
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_hint_timing(d, e5, 0, Some(TIMING_MAIN_END | TIMING_MSET | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e5, 1, ID, 1, 0);
    effect::set_condition(d, e5, Some(lua_fn(e5_cond)));
    effect::set_target(d, e5, Some(cost_target(postg)));
    effect::set_operation(d, e5, Some(operation(posop)));
    card::register_effect(d, c, e5, false);
}
fn indesconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) && !card::is_code(d, c, &[ID])
}
fn indescon(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
        || duel::is_existing_matching_card(d, Some(&indesconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MIMIGHOUL as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn e5_cond(d: &mut Duel, a: Args<'_>) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, 1 - tp) && duel::is_main_phase(d, None)
}
fn postfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_can_change_position(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&postfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, a.tp, 0);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_matching_card(d, tp, Some(&postfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::hint_selection(d, tc, true);
        let pos = duel::select_position(d, tp, tc, (POS_FACEUP_ATTACK | POS_FACEUP_DEFENSE) as u8);
        duel::change_position(d, tc, pos as u8, None, None, None, false);
    }
}
