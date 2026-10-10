//! Lord of the Heavenly Prison
use super::prelude::*;

pub const ID: u32 = 9822220;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal and protect all Set cards.
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(revcost)));
    effect::set_operation(d, e1, Some(operation(revop)));
    card::register_effect(d, c, e1, false);
    // Special Summon from hand and search if card was revealed.
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_label(d, e2, &[0]);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn is_rev(d: &mut Duel, c: CardId) -> bool {
    card::is_public(d, c) && card::get_flag_effect(d, c, ID) > 0
}
fn revcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_public(d, h);
    }
    true
}
fn revop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Register flag effect which will be used to check for protection/search.
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
    // Reveal until the end of opponent's turn.
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_PUBLIC);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
    card::register_effect(d, c, e1, false);
    // Protect Set cards on the field as long as this card is revealed.
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_condition(d, e2, Some(condition(protcon)));
    effect::set_target(d, e2, Some(effect_card_fn(protfilter)));
    effect::set_value(d, e2, 1);
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
    card::register_effect(d, c, e2, false);
}
fn protcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    is_rev(d, h)
}
fn protfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_facedown(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else { return false };
    let Some(rc) = effect::get_handler(d, re) else { return false };
    !card::is_status(d, rc, STATUS_ACT_FROM_HAND)
        && effect::is_active_type(d, re, TYPE_SPELL | TYPE_TRAP)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    if card::is_public(d, c) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_SPELL | TYPE_TRAP) && card::is_s_setable(d, c, false, None)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let wasrev = effect::get_label(d, a.e) == 1;
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && wasrev
        && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let Some(tc) = group::get_first(d, &g) else { return };
        duel::break_effect(d);
        if duel::s_set(d, a.tp, tc, None, true) == 0 {
            return;
        }
        // Banish it during the End Phase of the next turn.
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        let turn = duel::get_turn_count(d, None) as i64 + 1;
        effect::set_label(d, e1, &[turn]);
        effect::set_label_object(d, e1, Value::Card(tc));
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_condition(d, e1, Some(condition(rmcon)));
        effect::set_operation(d, e1, Some(operation(rmop)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        duel::register_effect(d, e1, a.tp);
        card::create_effect_relation(d, tc, e1);
    }
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        return (duel::get_turn_count(d, None) as i64) == effect::get_label(d, a.e);
    }
    effect::reset(d, a.e);
    false
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
