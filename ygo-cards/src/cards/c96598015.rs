//! Pot of Riches
use super::prelude::*;

pub const ID: u32 = 96598015;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK | CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);

    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    card::is_pendulum_summoned(d, a.c(0))
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    duel::register_effect(d, e1, a.tp);

    let e2 = effect::create_effect(d, h);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT | EFFECT_FLAG_OATH, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, a.tp);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    (a.int(3) as u32 & SUMMON_TYPE_PENDULUM) != SUMMON_TYPE_PENDULUM
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_PENDULUM)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_able_to_deck(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2)
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 3, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 3, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let nv = aux::necro_valley_filter(|d, c| filter(d, c));
    let g = duel::get_matching_group(d, Some(&nv), a.tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, NO_EX);
    if g.len() < 3 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let sg = group::select(d, &g, a.tp, 3, 3, false, NO_EX).nil();
    duel::send_to_deck(d, sg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    let og = duel::get_operated_group(d);
    if group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    if ct == 3 {
        duel::break_effect(d);
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
}
