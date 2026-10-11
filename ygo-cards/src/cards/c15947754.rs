//! Silent Wolf Calupo
use super::prelude::*;

pub const ID: u32 = 15947754;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip the top card of your Deck to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Your opponent calls the original type of the Equip Card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_HANDES + CATEGORY_TOHAND);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        let eqg = card::get_equip_group(d, c);
        group::is_exists(d, &eqg, &|d, cc| card::has_flag_effect(d, cc, ID, None), 1, NO_EX)
    })));
    effect::set_target(d, e3, Some(cost_target(calltg)));
    effect::set_operation(d, e3, Some(operation(callop)));
    card::register_effect(d, c, e3, false);
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let decktop = duel::get_decktop_group(d, a.tp, 1);
    let Some(ec) = group::get_first(d, &decktop) else { return };
    duel::disable_shuffle_check(d, true);
    if !card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, ec, Some(ID), Some(false)) {
        return;
    }
    // The equipped monster gains 500 ATK
    let e1 = effect::create_effect(d, ec);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 500);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, ec, e1, false);
    // Equip limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    effect::set_value_fn(d, e2, effect_card_fn(move |_d, _e, cc| cc == c));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, ec, e2, false);
}
fn calltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn callop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let eqg = card::get_equip_group(d, c);
    let eqg = group::filter(d, &eqg, &|d, cc| card::has_flag_effect(d, cc, ID, None), NO_EX);
    let Some(ec) = group::get_first(d, &eqg) else { return };
    let op = duel::select_option(d, 1 - a.tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    duel::confirm_cards(d, 1 - a.tp, ec);
    let res = (op == 0 && card::is_monster_card(d, ec)) || (op == 1 && card::is_spell_card(d, ec)) || (op == 2 && card::is_trap_card(d, ec));
    if res {
        // Send this card to the GY
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    } else {
        // Discard 1 random card from your opponent's hand, and if you do, return this card to the hand
        let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        if g.is_empty() {
            return;
        }
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        if duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None) > 0 {
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        }
    }
}
