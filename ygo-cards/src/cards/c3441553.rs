//! Shimmering Shatterer of the Sacred Beasts
use super::prelude::*;

pub const ID: u32 = 3441553;
const CARD_ILLUSION_GATE: u32 = 33017964;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ILLUSION_GATE], listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

/// `e:GetChainData().choice`
const F_CHOICE: u8 = chain::field::CARD;
/// `e:GetChainData().event_value`
const F_EVENT_VALUE: u8 = chain::field::CARD + 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // - When your opponent activates a card or effect in response to your "Sacred Beast" monster's effect activation: Negate that opponent's effect
    // - Reveal 1 Level 10 "Sacred Beast" monster in your hand; discard 1 card, and if you do, add 1 "Illusion Gate" from your Deck to your hand
    // - Shuffle 3 "Sacred Beast" Traps from your GY and/or banishment into the Deck, then target 1 card your opponent controls; destroy it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[10]) && card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && !card::is_public(d, c)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_ILLUSION_GATE]) && card::is_able_to_hand(d, c, None)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_trap(d, c) && card::is_faceup(d, c) && card::is_able_to_deck_as_cost(d, c)
}
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // When your opponent activates a card or effect in response to your "Sacred Beast" monster's effect activation: Negate that opponent's effect
    let ev_info = duel::check_event(d, EVENT_CHAINING);
    let option_1 = match &ev_info {
        Some(ev) => {
            let ch = (ev.event_value - 1) as u8;
            ev.event_player == 1 - a.tp
                && chain::is_triggering_player(d, ch, a.tp).truthy()
                && chain::is_triggering(d, ch, chain::Prop::SetCard, &[SET_SACRED_BEAST as i64]).truthy()
                && chain::is_triggering(d, ch, chain::Prop::Type, &[TYPE_MONSTER as i64]).truthy()
                && duel::is_chain_disablable(d, ev.event_value as u8)
        }
        None => false,
    };
    // Reveal 1 Level 10 "Sacred Beast" monster in your hand; discard 1 card, and if you do, add 1 "Illusion Gate" from your Deck to your hand
    let option_2 = duel::is_existing_matching_card(d, Some(&revealfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // Shuffle 3 "Sacred Beast" Traps from your GY and/or banishment into the Deck, then target 1 card your opponent controls; destroy it
    let option_3 = duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, NO_EX)
        && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    if a.chk == Some(0) {
        return option_1 || option_2 || option_3;
    }
    let choice = duel::select_effect(
        d,
        a.tp,
        &[(option_1, aux::stringid(ID, 1)), (option_2, aux::stringid(ID, 2)), (option_3, aux::stringid(ID, 3))],
    );
    let choice_val = choice.map_or(Value::Nil, |x| Value::Int(x as i64));
    chain::set_effect_data(d, e, None, F_CHOICE, choice_val);
    if choice == Some(1) {
        // When your opponent activates a card or effect in response to your "Sacred Beast" monster's effect activation: Negate that opponent's effect
        effect::set_category(d, e, CATEGORY_DISABLE);
        effect::set_property(d, e, 0, 0);
        let event_value = ev_info.map_or(0, |ev| ev.event_value);
        chain::set_effect_data(d, e, None, F_EVENT_VALUE, Value::Int(event_value as i64));
    } else if choice == Some(2) {
        // Reveal 1 Level 10 "Sacred Beast" monster in your hand; discard 1 card, and if you do, add 1 "Illusion Gate" from your Deck to your hand
        effect::set_category(d, e, CATEGORY_HANDES + CATEGORY_TOHAND + CATEGORY_SEARCH);
        effect::set_property(d, e, 0, 0);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&revealfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        duel::confirm_cards(d, 1 - a.tp, g);
        duel::shuffle_hand(d, a.tp);
    } else if choice == Some(3) {
        // Shuffle 3 "Sacred Beast" Traps from your GY and/or banishment into the Deck, then target 1 card your opponent controls; destroy it
        effect::set_category(d, e, CATEGORY_DESTROY);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 3, 3, false, NO_EX).nil();
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    }
    true
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        return chain_choice(d, e) == 3 && card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    let choice = chain_choice(d, e);
    if choice == 1 {
        // When your opponent activates a card or effect in response to your "Sacred Beast" monster's effect activation: Negate that opponent's effect
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    } else if choice == 2 {
        // Reveal 1 Level 10 "Sacred Beast" monster in your hand; discard 1 card, and if you do, add 1 "Illusion Gate" from your Deck to your hand
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else if choice == 3 {
        // Shuffle 3 "Sacred Beast" Traps from your GY and/or banishment into the Deck, then target 1 card your opponent controls; destroy it
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let choice = chain_choice(d, e);
    if choice == 1 {
        // When your opponent activates a card or effect in response to your "Sacred Beast" monster's effect activation: Negate that opponent's effect
        let ev = chain::effect_data(d, e, None, F_EVENT_VALUE).int();
        duel::negate_effect(d, ev as u8);
    } else if choice == 2 {
        // Reveal 1 Level 10 "Sacred Beast" monster in your hand; discard 1 card, and if you do, add 1 "Illusion Gate" from your Deck to your hand
        if duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) > 0 {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !g.is_empty() {
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - a.tp, g);
            }
        }
    } else if choice == 3 {
        // Shuffle 3 "Sacred Beast" Traps from your GY and/or banishment into the Deck, then target 1 card your opponent controls; destroy it
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
/// `e:GetChainData().choice` as a number (0 when unset)
fn chain_choice(d: &Duel, e: EffectId) -> i64 {
    chain::effect_data(d, e, None, F_CHOICE).int()
}
