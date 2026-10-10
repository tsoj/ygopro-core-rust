//! Mulcharmy Fuwalos
use super::prelude::*;

pub const ID: u32 = 42141493;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MULCHARMY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects for the rest of the turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(lua_fn(|d, a| duel::get_field_group_count(d, a.p(1), LOCATION_ONFIELD, 0) == 0)));
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
    // Keep track of the activations of a "Mulcharmy" monster's effect
    duel::add_custom_activity_counter(
        d,
        ID,
        ACTIVITY_CHAIN,
        lua_fn(|d, a| {
            let re = a.e(0);
            let rc = effect::get_handler(d, re).nil();
            !(card::is_set_card(d, rc, &[SET_MULCHARMY as u16]) && effect::is_monster_effect(d, re))
        }),
    );
}
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_discardable(d, c, None, None) && duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_CHAIN) < 2;
    }
    duel::send_to_grave(d, c, REASON_COST | REASON_DISCARD, None, None);
    // You can only activate 1 other "Mulcharmy" monster effect, the turn you activate this effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(|d, x| duel::get_custom_activity_count(d, ID, effect::get_handler_player(d, x.e(0)), ACTIVITY_CHAIN) >= 2)),
    );
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d, x| {
            let rc = effect::get_handler(d, x.e(1)).nil();
            card::is_set_card(d, rc, &[SET_MULCHARMY as u16]) && effect::is_monster_effect(d, x.e(1))
        }),
    );
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Draw 1 card each time your opponent Special Summons a monster(s) from the Deck and/or Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(drcon)));
    effect::set_operation(d, e1, Some(operation(drop_fn)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Shuffle random cards from your hand to the Deck during the End Phase
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_operation(d, e2, Some(operation(tdop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
}
fn drconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_summon_player(d, c, 1 - tp) && card::is_summon_location(d, c, LOCATION_DECK | LOCATION_EXTRA)
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = a.eg.clone().nil();
    group::is_exists(d, &g, &|d, c| drconfilter(d, c, a.tp), 1, NO_EX)
}
fn drop_fn(d: &mut Duel, a: &EffArgs) {
    if !duel::is_chain_solving(d) {
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    } else {
        let eff = effect::get_label_object(d, a.e).as_effect();
        if let Some(eff) = eff {
            if !effect::is_deleted(d, eff) {
                let label = effect::get_label(d, eff);
                effect::set_label(d, eff, &[label + 1]);
                return;
            }
        }
        let c = effect::get_handler(d, a.e).nil();
        // Draw cards when the current Chain Link finishes resolving
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        effect::set_operation(d, e1, Some(operation(chainsolvedop)));
        effect::set_label(d, e1, &[1]);
        effect::set_label_object(d, e1, Value::Effect(a.e));
        effect::set_reset(d, e1, RESET_CHAIN, 1);
        duel::register_effect(d, e1, a.tp);
        effect::set_label_object(d, a.e, Value::Effect(e1));
        // Reset "e1" and the label object of "e" at the end of the Chain Link
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        let e = a.e;
        effect::set_operation(
            d,
            e2,
            Some(lua_fn(move |d, _x| {
                effect::set_label_object(d, e, Value::Nil);
                effect::reset(d, e1);
                Value::Nil
            })),
        );
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        duel::register_effect(d, e2, a.tp);
    }
}
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let n = effect::get_label(d, a.e);
    duel::draw(d, a.tp, n as u32, REASON_EFFECT);
    effect::reset(d, a.e);
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, lo, Value::Nil);
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let dif = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) - (duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) + 6);
    if dif > 0 {
        let mut hg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
        group::match_(d, &mut hg, &|d, c| card::is_able_to_deck(d, c), NO_EX);
        let g = group::random_select(d, &hg, a.tp, dif as usize);
        if g.is_empty() {
            return;
        }
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
