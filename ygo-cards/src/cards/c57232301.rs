//! Shadow Severing Strike
use super::prelude::*;

pub const ID: u32 = 57232301;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Apply 1 of the following effects based on 1 of the card types banished
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| !duel::is_phase(d, PHASE_DAMAGE))));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_label(d, e1, &[0]);
    card::register_effect(d, c, e1, false);
    // Register cards banished
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1a, EVENT_REMOVE);
    effect::set_range(d, e1a, LOCATION_SZONE);
    effect::set_label_object(d, e1a, Value::Effect(e1));
    effect::set_operation(d, e1a, Some(operation(regop)));
    card::register_effect(d, c, e1a, false);
    effect::set_label_object(d, e1, Value::Effect(e1a));
    // Add 1 of your other banished cards to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn regfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD) && card::is_reason(d, c, REASON_COST)
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_COST) && effect::is_activated(d, a.re.nil()) && effect::is_monster_effect(d, a.re.nil())
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let lo = effect::get_label_object(d, e).as_effect().nil();
    if !effect::check_count_limit(d, lo, a.tp) {
        return;
    }
    let ok = match a.re {
        Some(re) => effect::is_activated(d, re) && effect::is_monster_effect(d, re),
        None => false,
    };
    if !ok {
        return;
    }
    let og = group::filter(d, &a.eg(), &regfilter, NO_EX);
    if !og.is_empty() {
        let mut types: u32 = 0;
        for tc in group::iter(d, &og) {
            types |= card::get_main_card_type(d, tc);
        }
        let label = effect::get_label(d, e);
        effect::set_label(d, e, &[label | types as i64]);
        let c = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(e), 0, a.tp, a.tp, 0);
    }
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let lo = effect::get_label_object(d, e).as_effect().nil();
    let lo_label = effect::get_label(d, lo);
    let types = if lo_label > 0 { lo_label } else { effect::get_label(d, e) };
    let b1 = (types & TYPE_MONSTER as i64) > 0
        && duel::is_existing_matching_card(d, Some(&card::is_negatable_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    let b2 = (types & TYPE_SPELL as i64) > 0 && duel::is_player_can_draw(d, a.tp, 2);
    let b3 = (types & TYPE_TRAP as i64) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    effect::set_label(d, lo, &[0]);
    effect::set_label(d, e, &[types]);
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    let mut cat: u64 = 0;
    if (types & TYPE_MONSTER as i64) > 0 {
        cat |= CATEGORY_DISABLE;
        duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    }
    if (types & TYPE_SPELL as i64) > 0 {
        duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    }
    if (types & TYPE_TRAP as i64) > 0 {
        cat |= CATEGORY_REMOVE;
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    }
    effect::set_category(d, e, cat);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let types = effect::get_label(d, a.e);
    let b1 = (types & TYPE_MONSTER as i64) > 0
        && duel::is_existing_matching_card(d, Some(&card::is_negatable_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    let b2 = (types & TYPE_SPELL as i64) > 0 && duel::is_player_can_draw(d, a.tp, 2);
    let b3 = (types & TYPE_TRAP as i64) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    if !(b1 || b2 || b3) {
        return;
    }
    let op = duel::select_effect(
        d,
        a.tp,
        &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3)), (b3, aux::stringid(ID, 4))],
    );
    if op == Some(1) {
        // Negate the effects of 1 face-up Spell/Trap on the field until the end of this turn
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&card::is_negatable_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        if let Some(tc) = tc {
            duel::hint_selection(d, CardSet::from_card(tc), true);
            let c = effect::get_handler(d, a.e).nil();
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        }
    } else if op == Some(2) {
        // Draw 2 cards
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    } else if op == Some(3) {
        // Banish 1 monster on the field
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::hint_selection(d, g.clone(), true);
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), a.tp, LOCATION_REMOVED, 0, 1, c.into());
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_REMOVED as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| card::is_able_to_hand(d, cc, None)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, exc).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
