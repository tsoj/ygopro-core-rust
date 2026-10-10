//! Radiant Typhoon Vision
use super::prelude::*;

pub const ID: u32 = 20508881;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MYSTICAL_SPACE_TYPHOON], listed_series: &[SET_RADIANT_TYPHOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // If this card is destroyed by the effect of "Mystical Space Typhoon": You can Set this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_condition(d, e2, Some(condition(e2cond)));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_MYSTICAL_SPACE_TYPHOON]) && card::is_able_to_hand(d, c, None)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None) && duel::is_player_can_draw(d, a.tp, 2);
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        let e = a.e;
        effect::set_category(d, e, CATEGORY_DRAW + CATEGORY_HANDES);
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, a.tp, 2);
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, a.tp, 1);
    } else if op == 2 {
        let e = a.e;
        effect::set_category(d, e, CATEGORY_SEARCH + CATEGORY_TOHAND);
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    true
}
fn discardfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_set_card(d, c, &[SET_RADIANT_TYPHOON as u16]) || card::is_quick_play_spell(d, c)) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    let tp = a.tp;
    if op == 1 {
        // Draw 2 cards, then if you have a "Radiant Typhoon" card or a Quick-Play Spell in your hand, discard 1 of them, or your entire hand if you don't
        if duel::draw(d, tp, 2, REASON_EFFECT) > 0 {
            duel::shuffle_hand(d, tp);
            duel::break_effect(d);
            let handg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
            if group::is_exists(d, &handg, &discardfilter, 1, NO_EX) {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
                let dg = group::filter_select(d, &handg, tp, &discardfilter, 1, 1, false, NO_EX).nil();
                if !dg.is_empty() {
                    duel::send_to_grave(d, &dg, REASON_EFFECT | REASON_DISCARD, None, None);
                }
            } else {
                duel::send_to_grave(d, &handg, REASON_EFFECT | REASON_DISCARD, None, None);
            }
        }
    } else if op == 2 {
        // Add 1 "Mystical Space Typhoon" from your Deck or GY to your hand
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let f = aux::necro_valley_filter(thfilter);
        let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
fn e2cond(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        None => false,
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_code(d, h, &[CARD_MYSTICAL_SPACE_TYPHOON])
        }
    }
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    if card::is_location(d, c, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::s_set(d, a.tp, c, None, true);
    }
}
